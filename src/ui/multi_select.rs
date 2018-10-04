use cursive::align::{Align, HAlign, VAlign};
use cursive::direction::Direction;
use cursive::event::{Callback, Event, EventResult, Key, MouseButton, MouseEvent};
use cursive::menu::MenuTree;
use cursive::rect::Rect;
use cursive::theme::ColorStyle;
use cursive::vec::Vec2;
use cursive::view::{Position, View};
use cursive::views::MenuPopup;
use cursive::Cursive;
use cursive::Printer;
use cursive::With;
use std::borrow::Borrow;
use std::cell::Cell;
use std::cmp::min;
use std::rc::Rc;
use unicode_width::UnicodeWidthStr;

/// View to select an item among a list.
///
/// It contains a list of values of type T, with associated labels.
///
/// # Examples
///
/// ```rust
/// # extern crate cursive;
/// # extern crate marcos;
/// # use cursive::Cursive;
/// # use cursive::views::{Dialog, TextView};
/// # use cursive::align::HAlign;
/// # use marcos::ui::multi_select::MultiSelectView;
/// # fn main() {
/// let mut time_select = MultiSelectView::new().h_align(HAlign::Center);
/// time_select.add_item("Short", 1);
/// time_select.add_item("Medium", 5);
/// time_select.add_item("Long", 10);
///
/// time_select.set_on_submit(|s, time| {
///     s.pop_layer();
///     let text = format!("You will wait for {} minutes...", time);
///     s.add_layer(Dialog::around(TextView::new(text))
///                     .button("Quit", |s| s.quit()));
/// });
///
/// let mut siv = Cursive::dummy();
/// siv.add_layer(Dialog::around(time_select)
///                      .title("How long is your wait?"));
/// # }
///
/// ```
pub struct MultiSelectView<T = String> {
    items: Vec<Item<T>>,
    enabled: bool,
    // the focus needs to be manipulable from callbacks
    focus: Rc<Cell<usize>>,
    // This is a custom callback to include a &T.
    // It will be called whenever "Enter" is pressed.
    on_submit: Option<Rc<Fn(&mut Cursive, &T)>>,
    // This callback is called when the selection is changed.
    on_select: Option<Rc<Fn(&mut Cursive, &T)>>,
    align: Align,
    // `true` if we show a one-line view, with popup on selection.
    popup: bool,
    // We need the last offset to place the popup window
    // We "cache" it during the draw, so we need interior mutability.
    last_offset: Cell<Vec2>,
    last_size: Vec2,
    input_buffer: Vec<Event>,
    input_num_buffer: Vec<usize>,
    input_count: usize,
}

impl<T: 'static> Default for MultiSelectView<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: 'static> MultiSelectView<T> {
    /// Creates a new empty SelectView.
    pub fn new() -> Self {
        MultiSelectView {
            items: Vec::new(),
            enabled: true,
            focus: Rc::new(Cell::new(0)),
            on_select: None,
            on_submit: None,
            align: Align::top_left(),
            popup: false,
            last_offset: Cell::new(Vec2::zero()),
            last_size: Vec2::zero(),
            input_buffer: Vec::new(),
            input_num_buffer: Vec::new(),
            input_count: 0,
        }
    }

    /// Turns `self` into a popup select view.
    ///
    /// Chainable variant.
    pub fn popup(self) -> Self {
        self.with(|s| s.set_popup(true))
    }

    /// Turns `self` into a popup select view.
    pub fn set_popup(&mut self, popup: bool) {
        self.popup = popup;
    }

    /// Disables this view.
    ///
    /// A disabled view cannot be selected.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Disables this view.
    ///
    /// Chainable variant.
    pub fn disabled(self) -> Self {
        self.with(Self::disable)
    }

    /// Re-enables this view.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Enable or disable this view.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Returns `true` if this view is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Sets a callback to be used when an item is selected.
    pub fn set_on_select<F>(&mut self, cb: F)
    where
        F: Fn(&mut Cursive, &T) + 'static,
    {
        self.on_select = Some(Rc::new(cb));
    }

    /// Sets a callback to be used when an item is selected.
    ///
    /// Chainable variant.
    pub fn on_select<F>(self, cb: F) -> Self
    where
        F: Fn(&mut Cursive, &T) + 'static,
    {
        self.with(|s| s.set_on_select(cb))
    }

    /// Sets a callback to be used when `<Enter>` is pressed.
    ///
    /// The item currently selected will be given to the callback.
    ///
    /// Here, `V` can be `T` itself, or a type that can be borrowed from `T`.
    pub fn set_on_submit<F, R, V: ?Sized>(&mut self, cb: F)
    where
        F: 'static + Fn(&mut Cursive, &V) -> R,
        T: Borrow<V>,
    {
        self.on_submit = Some(Rc::new(move |s, t| {
            cb(s, t.borrow());
        }));
    }

    /// Sets a callback to be used when `<Enter>` is pressed.
    ///
    /// The item currently selected will be given to the callback.
    ///
    /// Chainable variant.
    pub fn on_submit<F, V: ?Sized>(self, cb: F) -> Self
    where
        F: Fn(&mut Cursive, &V) + 'static,
        T: Borrow<V>,
    {
        self.with(|s| s.set_on_submit(cb))
    }

    /// Sets the alignment for this view.
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;

        self
    }

    /// Sets the vertical alignment for this view.
    /// (If the view is given too much space vertically.)
    pub fn v_align(mut self, v: VAlign) -> Self {
        self.align.v = v;

        self
    }

    /// Sets the horizontal alignment for this view.
    pub fn h_align(mut self, h: HAlign) -> Self {
        self.align.h = h;

        self
    }

    /// Returns the value of the currently selected item.
    ///
    /// Returns `None` if the list is empty.
    pub fn selection(&self) -> Option<Rc<T>> {
        let focus = self.focus();
        if self.len() <= focus {
            None
        } else {
            Some(Rc::clone(&self.items[focus].value))
        }
    }

    /// Removes all items from this view.
    pub fn clear(&mut self) {
        self.items.clear();
        self.focus.set(0);
    }

    /// Adds a item to the list, with given label and value.
    pub fn add_item<S: Into<String>>(&mut self, label: S, value: T) {
        self.items.push(Item::new(label.into(), value));
    }

    /// Gets an item at given idx or None.
    ///
    /// ```
    /// extern crate cursive;
    /// extern crate marcos;
    /// use cursive::Cursive;
    /// use cursive::views::{TextView};
    /// use marcos::ui::multi_select::MultiSelectView;
    /// let select = MultiSelectView::new()
    ///     .item("Short", 1);
    /// assert_eq!(select.get_item(0), Some(("Short", &1)));
    /// ```
    pub fn get_item(&self, i: usize) -> Option<(&str, &T)> {
        self.items
            .get(i)
            .map(|item| (item.label.as_ref(), &*item.value))
    }

    /// Gets a mut item at given idx or None.
    pub fn get_item_mut(&mut self, i: usize) -> Option<(&mut String, &mut T)> {
        if i >= self.items.len() {
            None
        } else {
            let item = &mut self.items[i];
            if let Some(t) = Rc::get_mut(&mut item.value) {
                let label = &mut item.label;
                Some((label, t))
            } else {
                None
            }
        }
    }

    /// Removes an item from the list.
    ///
    /// Returns a callback in response to the selection change.
    ///
    /// You should run this callback with a `&mut Cursive`.
    pub fn remove_item(&mut self, id: usize) -> Callback {
        self.items.remove(id);
        let focus = self.focus();
        if focus >= id && focus > 0 {
            self.focus.set(focus - 1);
        }

        self.make_select_cb().unwrap_or_else(Callback::dummy)
    }

    /// Inserts an item at position `index`, shifting all elements after it to
    /// the right.
    pub fn insert_item<S>(&mut self, index: usize, label: S, value: T)
    where
        S: Into<String>,
    {
        self.items.insert(index, Item::new(label.into(), value));
    }

    /// Chainable variant of add_item
    pub fn item<S: Into<String>>(self, label: S, value: T) -> Self {
        self.with(|s| s.add_item(label, value))
    }

    /// Adds all items from from an iterator.
    pub fn add_all<S, I>(&mut self, iter: I)
    where
        S: Into<String>,
        I: IntoIterator<Item = (S, T)>,
    {
        for (s, t) in iter {
            self.add_item(s, t);
        }
    }

    /// Adds all items from from an iterator.
    ///
    /// Chainable variant.
    pub fn with_all<S, I>(self, iter: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = (S, T)>,
    {
        self.with(|s| s.add_all(iter))
    }

    fn draw_item(&self, printer: &Printer, i: usize) {
        let l = self.items[i].label.width();
        let x = self.align.h.get_offset(l, printer.size.x);
        printer.print_hline((0, 0), x, " ");
        printer.print((x, 0), &self.items[i].label);
        if l < printer.size.x {
            assert!((l + x) <= printer.size.x);
            printer.print_hline((x + l, 0), printer.size.x - (l + x), " ");
        }
    }

    /// Returns the id of the item currently selected.
    ///
    /// Returns `None` if the list is empty.
    pub fn selected_id(&self) -> Option<usize> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.focus())
        }
    }

    /// Returns the number of items in this list.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` if this list has no item.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn focus(&self) -> usize {
        self.focus.get()
    }

    /// Moves the selection to the given position.
    ///
    /// Returns a callback in response to the selection change.
    ///
    /// You should run this callback with a `&mut Cursive`.
    pub fn set_selection(&mut self, i: usize) -> Callback {
        // TODO: Check if `i >= self.len()` ?
        // assert!(i < self.len(), "SelectView: trying to select out-of-bound");
        // Or just cap the ID?
        let i = if self.is_empty() {
            0
        } else {
            min(i, self.len() - 1)
        };
        self.focus.set(i);

        self.make_select_cb().unwrap_or_else(Callback::dummy)
    }

    /// Sets the selection to the given position.
    ///
    /// Chainable variant.
    ///
    /// Does not apply `on_select` callbacks.
    pub fn selected(self, i: usize) -> Self {
        self.with(|s| {
            s.set_selection(i);
        })
    }

    /// Moves the selection up by the given number of rows.
    ///
    /// Returns a callback in response to the selection change.
    ///
    /// You should run this callback with a `&mut Cursive`:
    ///
    /// ```rust
    /// # extern crate cursive;
    /// # extern crate marcos;
    /// # use cursive::Cursive;
    /// # use marcos::ui::multi_select::MultiSelectView;
    /// # fn main() {}
    /// fn select_up(siv: &mut Cursive, view: &mut MultiSelectView<()>) {
    ///     let cb = view.select_up(1);
    ///     cb(siv);
    /// }
    /// ```
    pub fn select_up(&mut self, n: usize) -> Callback {
        self.focus_up(n);
        self.make_select_cb().unwrap_or_else(Callback::dummy)
    }

    /// Moves the selection down by the given number of rows.
    ///
    /// Returns a callback in response to the selection change.
    ///
    /// You should run this callback with a `&mut Cursive`.
    pub fn select_down(&mut self, n: usize) -> Callback {
        self.focus_down(n);
        self.make_select_cb().unwrap_or_else(Callback::dummy)
    }

    fn focus_up(&mut self, n: usize) {
        let focus = self.focus().saturating_sub(n);
        self.focus.set(focus);
    }

    fn focus_down(&mut self, n: usize) {
        let focus = min(self.focus() + n, self.items.len().saturating_sub(1));
        self.focus.set(focus);
    }

    fn submit(&mut self) -> EventResult {
        let cb = self.on_submit.clone().unwrap();
        // We return a Callback Rc<|s| cb(s, &*v)>
        EventResult::Consumed(
            self.selection()
                .map(|v| Callback::from_fn(move |s| cb(s, &v))),
        )
    }

    fn on_event_regular(&mut self, event: Event) -> EventResult {
        if is_numeric(&event) {
            let number: Option<u32> = match event {
                Event::Char(c) => c.to_digit(10),
                _ => None,
            };
            if let Some(c) = number {
                self.input_num_buffer.push(c as usize)
            };
            return EventResult::Ignored;
        }

        if should_intercept(&event) {
            self.input_buffer.push(event.clone())
        }
        match self.input_buffer.as_slice() {
            [Event::Char('g'), Event::Char('g')] => {
                self.focus.set(0);
                self.input_buffer.clear();
                return EventResult::Consumed(self.make_select_cb());
            }
            [Event::Char('G')] => {
                let num = get_number(&self.input_num_buffer);
                if num == 0 {
                } else {
                    if num - 1 < self.items.len() {
                        self.focus.set(num - 1);
                    } else {
                        self.focus.set(self.items.len().saturating_sub(1));
                    }
                    self.input_buffer.clear();
                    self.input_num_buffer.clear();
                    return EventResult::Consumed(self.make_select_cb());
                }
            }
            _ => {}
        }
        match event {
            Event::Key(Key::Esc) => self.input_num_buffer.clear(),
            Event::Key(Key::Up) if self.focus() > 0 => self.focus_up(1),
            Event::Key(Key::Down) if self.focus() + 1 < self.items.len() => self.focus_down(1),
            Event::Key(Key::PageUp) => self.focus_up(10),
            Event::Key(Key::PageDown) => self.focus_down(10),
            Event::Key(Key::Home) => self.focus.set(0),
            Event::Key(Key::End) => self.focus.set(self.items.len().saturating_sub(1)),
            Event::Char('G') => self.focus.set(self.items.len().saturating_sub(1)),
            // Event::Char('/') => {
            //     debug!("You pressed search key!");
            // },
            Event::Mouse {
                event: MouseEvent::Press(_),
                position,
                offset,
            }
                if position
                    .checked_sub(offset)
                    .map(|position| position < self.last_size && position.y < self.len())
                    .unwrap_or(false) =>
            {
                self.focus.set(position.y - offset.y)
            }
            Event::Mouse {
                event: MouseEvent::Release(MouseButton::Left),
                position,
                offset,
            }
                if self.on_submit.is_some() && position
                    .checked_sub(offset)
                    .map(|position| position < self.last_size && position.y == self.focus())
                    .unwrap_or(false) =>
            {
                return self.submit();
            }
            Event::Key(Key::Enter) if self.on_submit.is_some() => {
                return self.submit();
            }
            // Event::Char(c) => {
            //     // Starting from the current focus,
            //     // find the first item that match the char.
            //     // Cycle back to the beginning of
            //     // the list when we reach the end.
            //     // This is achieved by chaining twice the iterator
            //     let iter = self.items.iter().chain(self.items.iter());
            //     if let Some((i, _)) = iter
            //         .enumerate()
            //         .skip(self.focus() + 1)
            //         .find(|&(_, item)| item.label.starts_with(c))
            //     {
            //         // Apply modulo in case we have a hit
            //         // from the chained iterator
            //         self.focus.set(i % self.items.len());
            //     } else {
            //         return EventResult::Ignored;
            //     }
            // }
            _ => return EventResult::Ignored,
        }

        EventResult::Consumed(self.make_select_cb())
    }

    /// Returns a callback from selection change.
    fn make_select_cb(&self) -> Option<Callback> {
        self.on_select.clone().and_then(|cb| {
            self.selection()
                .map(|v| Callback::from_fn(move |s| cb(s, &v)))
        })
    }

    fn open_popup(&mut self) -> EventResult {
        // Build a shallow menu tree to mimick the items array.
        // TODO: cache it?
        let mut tree = MenuTree::new();
        for (i, item) in self.items.iter().enumerate() {
            let focus = Rc::clone(&self.focus);
            let on_submit = self.on_submit.as_ref().cloned();
            let value = Rc::clone(&item.value);
            tree.add_leaf(item.label.clone(), move |s| {
                // TODO: What if an item was removed in the meantime?
                focus.set(i);
                if let Some(ref on_submit) = on_submit {
                    on_submit(s, &value);
                }
            });
        }
        // Let's keep the tree around,
        // the callback will want to use it.
        let tree = Rc::new(tree);

        let focus = self.focus();
        // This is the offset for the label text.
        // We'll want to show the popup so that the text matches.
        // It'll be soo cool.
        let item_length = self.items[focus].label.len();
        let text_offset = (self.last_size.x.saturating_sub(item_length)) / 2;
        // The total offset for the window is:
        // * the last absolute offset at which we drew this view
        // * shifted to the right of the text offset
        // * shifted to the top of the focus (so the line matches)
        // * shifted top-left of the border+padding of the popup
        let offset = self.last_offset.get();
        let offset = offset + (text_offset, 0);
        let offset = offset.saturating_sub((0, focus));
        let offset = offset.saturating_sub((2, 1));

        // And now, we can return the callback that will create the popup.
        EventResult::with_cb(move |s| {
            // The callback will want to work with a fresh Rc
            let tree = Rc::clone(&tree);
            // We'll relativise the absolute position,
            // So that we are locked to the parent view.
            // A nice effect is that window resizes will keep both
            // layers together.
            let current_offset = s.screen().offset();
            let offset = offset.signed() - current_offset;
            // And finally, put the view in view!
            s.screen_mut()
                .add_layer_at(Position::parent(offset), MenuPopup::new(tree).focus(focus));
        })
    }

    // A popup view only does one thing: open the popup on Enter.
    fn on_event_popup(&mut self, event: Event) -> EventResult {
        match event {
            // TODO: add Left/Right support for quick-switch?
            Event::Key(Key::Enter) => self.open_popup(),
            Event::Mouse {
                event: MouseEvent::Release(MouseButton::Left),
                position,
                offset,
            }
                if position.fits_in_rect(offset, self.last_size) =>
            {
                self.open_popup()
            }
            _ => EventResult::Ignored,
        }
    }
}

impl MultiSelectView<String> {
    /// Convenient method to use the label as value.
    pub fn add_item_str<S: Into<String>>(&mut self, label: S) {
        let label = label.into();
        self.add_item(label.clone(), label);
    }

    /// Chainable variant of add_item_str
    pub fn item_str<S: Into<String>>(self, label: S) -> Self {
        self.with(|s| s.add_item_str(label))
    }

    /// Convenient method to use the label as value.
    pub fn insert_item_str<S>(&mut self, index: usize, label: S)
    where
        S: Into<String>,
    {
        let label = label.into();
        self.insert_item(index, label.clone(), label);
    }

    /// Adds all strings from an iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate marcos;
    /// # use marcos::ui::multi_select::MultiSelectView;
    /// let mut select_view = MultiSelectView::new();
    /// select_view.add_all_str(vec!["a", "b", "c"]);
    /// ```
    pub fn add_all_str<S, I>(&mut self, iter: I)
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        for s in iter {
            self.add_item_str(s);
        }
    }

    /// Adds all strings from an iterator.
    ///
    /// Chainable variant.
    pub fn with_all_str<S, I>(self, iter: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        self.with(|s| s.add_all_str(iter))
    }
}

impl<T: 'static> View for MultiSelectView<T> {
    fn draw(&self, printer: &Printer) {
        self.last_offset.set(printer.offset);

        if self.popup {
            // Popup-select only draw the active element.
            // We'll draw the full list in a popup if needed.
            let style = if !self.enabled {
                ColorStyle::secondary()
            } else if !printer.focused {
                ColorStyle::primary()
            } else {
                ColorStyle::highlight()
            };
            let x = match printer.size.x.checked_sub(1) {
                Some(x) => x,
                None => return,
            };

            printer.with_color(style, |printer| {
                // Prepare the entire background
                printer.print_hline((1, 0), x, " ");
                // Draw the borders
                printer.print((0, 0), "<");
                printer.print((x, 0), ">");

                let label = &self.items[self.focus()].label;

                // And center the text?
                let offset = HAlign::Center.get_offset(label.len(), x + 1);

                printer.print((offset, 0), label);
            });
        } else {
            // Non-popup mode: we always print the entire list.
            let h = self.items.len();
            let offset = self.align.v.get_offset(h, printer.size.y);
            let printer = &printer.offset((0, offset));

            for i in 0..self.len() {
                printer
                    .offset((0, i))
                    .with_selection(i == self.focus(), |printer| {
                        if i != self.focus() && !self.enabled {
                            printer.with_color(ColorStyle::secondary(), |printer| {
                                self.draw_item(printer, i)
                            });
                        } else {
                            self.draw_item(printer, i);
                        }
                    });
            }
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        // Items here are not compressible.
        // So no matter what the horizontal requirements are,
        // we'll still return our longest item.
        let w = self
            .items
            .iter()
            .map(|item| item.label.width())
            .max()
            .unwrap_or(1);
        if self.popup {
            Vec2::new(w + 2, 1)
        } else {
            let h = self.items.len();

            Vec2::new(w, h)
        }
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        if self.popup {
            self.on_event_popup(event)
        } else {
            self.on_event_regular(event)
        }
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        self.enabled && !self.items.is_empty()
    }

    fn layout(&mut self, size: Vec2) {
        self.last_size = size;
    }

    fn important_area(&self, size: Vec2) -> Rect {
        self.selected_id()
            .map(|i| Rect::from_size((0, i), (size.x, 1)))
            .unwrap_or_else(|| Rect::from((0, 0)))
    }
}

struct Item<T> {
    label: String,
    value: Rc<T>,
}

impl<T> Item<T> {
    fn new(label: String, value: T) -> Self {
        Item {
            label,
            value: Rc::new(value),
        }
    }
}

fn is_numeric(event: &Event) -> bool {
    match event {
        Event::Char(c) => if c.is_numeric() {
            return true;
        },
        _ => {}
    }
    false
}

fn is_alphabete(event: &Event) -> bool {
    match event {
        Event::Char(c) => if c.is_alphabetic() {
            return true;
        },
        _ => {}
    }
    false
}

fn get_number(seq: &Vec<usize>) -> usize {
    let mut ans = 0usize;
    let mut len = seq.len() as u32;
    for i in seq.into_iter() {
        ans += i * 10usize.pow(len - 1);
        len -= 1;
    }
    ans
}

fn should_intercept(event: &Event) -> bool {
    match event {
        Event::Char('g') => return true,
        Event::Char('G') => return true,
        _ => {}
    }
    false
}
