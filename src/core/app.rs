//! Module contains functions related to core functionalities of the app.

use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs as stdfs;
use std::path::PathBuf;
use std::process;
use std::rc::Rc;

use cursive::event::{Event, EventResult, Key};
#[allow(unused_imports)]
use cursive::traits::{Boxable, Identifiable, Scrollable};
use cursive::views::*;
use cursive::align::*;
use cursive::theme::*;
use cursive::view::Position;
use cursive::Cursive;

use dirs;

use alphanumeric_sort::compare_os_str;
use walkdir;
use walkdir::WalkDir;

use mime_guess::guess_mime_type;
use mime_guess::Mime;

use crate::config;
use crate::error::*;
use crate::fs::Entry;
use crate::ui::MultiSelectView;
use crate::ui::Tab;
use crate::utils::{filter, info, logger};

/// Create a new instance of marcos with the specified backend.
///
/// It also setups the logger for log events
pub fn init(path: &str, log_file: Option<&str>, log_level: Option<&str>) -> Result<App> {
    logger::init(log_file, log_level)?;
    let path = match path {
        "." | "./" => env::current_dir()?,
        "../" | ".." => env::current_dir()?.parent().unwrap().to_path_buf(),
        x => PathBuf::from(x),
    };
    info!("Initializing with path {:?}", path);
    if !path.is_dir() {
        debug!("Failure with directory {:?}", path);
        println!("Incorrect path or unaccessible directory! Please cheack PATH");
        process::exit(1);
    }
    let _app_config = config::Config::load();
    let mut app = App::new()?;
    app.add_tab(1, path)?;
    app.load_bindings();
    Ok(app)
}

/// The data structure holding various elements related to `App`.
#[allow(dead_code)]
pub struct App {
    /// The main application, the cursive instance.
    pub siv: Cursive,
    /// The vector of tabs
    // pub vec_tabs: HashMap<String, Tab>,
    pub vec_tabs: Rc<RefCell<HashMap<u32, Tab>>>,
    /// The index of focused tab starting from 0.
    focused_tab: usize,
    /// The index of focused entry starting from 0.
    focused_entry: usize,
}

impl App {
    /// Create a new instance of cursive with default global callbacks.
    /// `q` is used to quit the cursive instance.
    ///
    /// TODO `:` is used to open the command box
    pub fn new() -> Result<Self> {
        let data_path: PathBuf = dirs::config_dir().ok_or(ErrorKind::DirNotFound {
            dirname: String::from("CONFIG_DIR"),
        })?;
        let data_path = data_path.join("marcos");
        if !data_path.exists() {
            stdfs::create_dir_all(&data_path).expect("Cannot create data_dir");
        }
        let asset_file = data_path.join("style.toml");
        debug!("Loading theme from file: {:?}", asset_file);
        if !asset_file.is_file() {
            stdfs::File::create(&asset_file).expect("Failed to create asset file");
        }
        let mut siv = Cursive::default();

        // Create empty views
        let p_widget = MultiSelectView::<PathBuf>::new().with_id("parent");
        let c_widget = MultiSelectView::<PathBuf>::new().on_select(update_info);
        let c_widget = OnEventView::new(c_widget).with_id("current");
        let preview_widget = TextView::new("").with_id("preview");
        let top_widget = LinearLayout::horizontal()
            .child(TextView::new(info::user_info())
                   .h_align(HAlign::Left)
                   .with_id("topbar/left"))
            .child(TextView::new(info::user_info())
                   .h_align(HAlign::Left)
                   .with_id("topbar/center").full_width())
            .child(TextView::new(info::disk_info("/"))
                   .h_align(HAlign::Right)
                   .with_id("topbar/right").full_width());
        // let top_bar = TextView::new(format!("{} {}", info::user_info(), info::disk_info("/")))
        //     .with_id("topbar");
        let mut status_bar = HideableView::new(TextView::new("Status").with_id("status"));
        status_bar.unhide();
        // let console = EditView::new().filler(">").with_id("console");
        // let console = HideableView::new(console);
        // let console = console.with_id("console/vis");
        let status_bar = status_bar.with_id("status/vis");

        // Horizontal panes
        let mut panes = LinearLayout::horizontal();
        panes.add_child(
            Panel::new(p_widget)
                .full_width()
                .max_width(30)
                .full_height(),
        );
        panes.add_child(
            Panel::new(c_widget)
                .full_width()
                .max_width(40)
                .full_height(),
        );
        panes.add_child(Panel::new(preview_widget).full_width().full_height());
        let h_panes = LinearLayout::vertical()
            .child(top_widget.full_width())
            .child(panes)
            // .child(console)
            .child(status_bar);

        siv.add_layer(h_panes);
        siv.add_global_callback(Event::CtrlChar('w'), |s| s.quit());
        siv.add_global_callback('q', |s| s.quit());
        let vec_tabs = Rc::new(RefCell::new(HashMap::<u32, Tab>::new()));

        debug!("Loading theme resource file");
        siv.load_theme_file(asset_file).expect("Cannot find file!");
        Ok(Self {
            siv,
            vec_tabs,
            focused_entry: 0,
            focused_tab: 0,
        })
    }

    /// Funtion to load key-bindings.
    pub fn load_bindings(&mut self) {
        let v_clone = self.vec_tabs.clone();
        self.siv.add_global_callback('h', move |s: &mut Cursive| {
            debug!("Inside global callback h");
            // Get current_view selection index
            let mut current_selection = None;
            s.call_on_id(
                "current",
                |event_view: &mut OnEventView<MultiSelectView<PathBuf>>| {
                    let view = event_view.get_inner();
                    current_selection = view.selected_id();
                },
            );
            if let Some(mut tab) = v_clone.borrow_mut().get_mut(&1) {
                tab.focused
                    .insert(PathBuf::from(&tab.c_view), current_selection.unwrap_or(0));
                tab.go_back();
                // tab.c_focused = current_selection;
                App::update_tab(s, &mut tab);
                s.call_on_id("topbar/center", |view: &mut TextView| {
                    let mut text: TextContent = view.get_shared_content();
                    text.set_content(format!(" {}", tab.c_view.to_str().unwrap()));
                });
            };
        });

        let v_clone2 = self.vec_tabs.clone();
        self.siv.add_global_callback('l', move |s: &mut Cursive| {
            s.call_on_id(
                "current",
                |event_view: &mut OnEventView<MultiSelectView<PathBuf>>| {
                    let event = event_view.get_inner_mut();
                    if let Some(path) = event.selection() {
                        if path.is_dir() {
                            if let Some(mut tab) = v_clone2.borrow_mut().get_mut(&1) {
                                debug!("Moving forward to path {:?}", path);
                                tab.go_forward(path.to_path_buf());
                            };
                        } // if
                    };
                },
            );
            if let Some(mut tab) = v_clone2.borrow_mut().get_mut(&1) {
                App::update_tab(s, &mut tab);
                s.call_on_id("topbar/center", |view: &mut TextView| {
                    let mut text = view.get_shared_content();
                    text.set_content(format!(" {}",tab.c_view.to_str().unwrap()));
                });
            }
        });

        self.siv.call_on_id(
            "current",
            |event_view: &mut OnEventView<MultiSelectView<PathBuf>>| {
                event_view.set_on_pre_event_inner('k', |s| {
                    let cb = s.select_up(1);
                    Some(EventResult::Consumed(Some(cb)))
                });
                event_view.set_on_pre_event_inner('j', |s| {
                    let cb = s.select_down(1);
                    Some(EventResult::Consumed(Some(cb)))
                });
            },
        );

        self.siv.add_global_callback('/', |c: &mut Cursive| {
            debug!("You pressed search key");
            c.add_layer(create_console("/"));
            let s = c.screen_mut();
            let l = LayerPosition::FromFront(0);
            let pos = s.offset().saturating_add((9000, 9000));
            let p = Position::absolute(pos);
            s.reposition_layer(l, p);
        });

        self.siv.add_global_callback('?', |c: &mut Cursive| {
            debug!("You pressed search key");
            c.add_layer(create_console("?"));
            let s = c.screen_mut();
            let l = LayerPosition::FromFront(0);
            let pos = s.offset().saturating_add((9000, 9000));
            let p = Position::absolute(pos);
            s.reposition_layer(l, p);
        });

        // Cancels current action.
        self.siv.add_global_callback(Event::Key(Key::Esc), |s: &mut Cursive| {
            let mut exists: bool = false;
            {
                let stack_view = s.screen_mut();
                if let Some(_data) = stack_view.get(LayerPosition::FromBack(1)) {
                    debug!("Multiple layer found");
                    exists = true;
                }
            }
            if exists {s.pop_layer();}
        });
    }

    /// [Experimental] Adds a new tab to the main view. Currently only single tab is supported
    /// for the sake of simplicity. Multiple tabs support will land in near future.
    pub fn add_tab(&mut self, name: u32, path: PathBuf) -> Result<()> {
        let mut tab = Tab::from(name, &path)?;
        self.siv.call_on_id("topbar/center", |view: &mut TextView| {
            let mut current_text: TextContent = view.get_shared_content();
            current_text.set_content(format!(" {}", path.to_str().unwrap()));
        });
        self.siv.call_on_id(
            "current",
            |event_view: &mut OnEventView<MultiSelectView<PathBuf>>| {
                let view = event_view.get_inner_mut();
                view.clear();
                for entry in App::get_path_iter(&tab.c_view)
                    .filter_entry(|e| e.path().is_dir() && !filter::is_hidden(e))
                {
                    let entry = entry.unwrap();
                    match entry.file_name().to_str() {
                        Some(c) => {
                            view.add_item(format!(r"  {}", c), PathBuf::from(entry.path()))
                        }
                        None => {}
                    }
                }
                for entry in App::get_path_iter(&tab.c_view)
                    .filter_entry(|e| e.path().is_file() && !filter::is_hidden(e))
                {
                    let entry = entry.unwrap();
                    match entry.file_name().to_str() {
                        Some(c) => {
                            view.add_item(format!(r"  {}", c), PathBuf::from(entry.path()))
                        }
                        None => {}
                    };
                }
                view.set_selection(0);
            },
        );

        let mut i: usize = 0;
        self.siv
            .call_on_id("parent", |view: &mut MultiSelectView<PathBuf>| {
                view.clear();
                debug!("siv call on id parent {:?}", tab.p_view.to_str());
                match tab.p_view.to_str() {
                    Some("root") => {
                        view.add_item("/", PathBuf::from("/"));
                        view.set_enabled(false);
                        view.set_selection(0);
                    }
                    Some(_) | None => {
                        for (index, entry) in App::get_path_iter(&tab.p_view)
                            .filter_entry(|e| e.path().is_dir() && !filter::is_hidden(e))
                            .enumerate()
                        {
                            let entry = entry.unwrap();
                            if entry.path() == &tab.c_view {
                                i = index;
                            }
                            match entry.file_name().to_str() {
                                Some(c) => view
                                    .add_item(format!("  {}", c), PathBuf::from(entry.path())),
                                None => {}
                            };
                        }
                        for entry in App::get_path_iter(&tab.p_view)
                            .filter_entry(|e| e.path().is_file() && !filter::is_hidden(e))
                        {
                            let entry = entry.unwrap();
                            match entry.file_name().to_str() {
                                Some(c) => view
                                    .add_item(format!("  {}", c), PathBuf::from(entry.path())),
                                None => {}
                            };
                        }
                        view.set_selection(i);
                        view.set_enabled(false);
                    } // None
                };
            });
        tab.focused.insert(PathBuf::from(&tab.p_view), i);
        // tab.p_focused = i;
        debug!("Value of tab: {:?}", tab);
        self.vec_tabs.borrow_mut().insert(1, tab);
        self.focused_entry = i;
        debug!("Value of arr: {:?}", self.vec_tabs.borrow());
        Ok(())
    }

    /// Funtion which updates the content of `Tab` when you go forward or
    /// backward in a hierarchy.
    fn update_tab(siv: &mut Cursive, tab: &mut Tab) {
        // let focused = if !forward { tab.p_focused } else {
        //     if let Some(c) = tab.c_focused {c}
        //     else {0}
        // };
        siv.call_on_id(
            "current",
            |event_view: &mut OnEventView<MultiSelectView<PathBuf>>| {
                let c_focused = tab.focused.get(&tab.c_view).unwrap_or(&0usize);
                debug!("Got current selection of: {:?}", c_focused);
                let view = event_view.get_inner_mut();
                view.clear();
                for entry in App::get_path_iter(&tab.c_view)
                    .filter_entry(|e| e.path().is_dir() && !filter::is_hidden(e))
                {
                    let entry = entry.unwrap();
                    match entry.file_name().to_str() {
                        Some(c) => {
                            view.add_item(format!(r"  {}", c), PathBuf::from(entry.path()))
                        }
                        None => {}
                    }
                }
                for entry in App::get_path_iter(&tab.c_view)
                    .filter_entry(|e| e.path().is_file() && !filter::is_hidden(e))
                {
                    let entry = entry.unwrap();
                    match entry.file_name().to_str() {
                        Some(c) => {
                            view.add_item(format!(r"  {}", c), PathBuf::from(entry.path()))
                        }
                        None => {}
                    };
                }
                // TODO keep last selection
                view.set_selection(*c_focused);
                //view.set_selection(focused);
            },
        );

        let mut i: usize = 0;
        siv.call_on_id("parent", |view: &mut MultiSelectView<PathBuf>| {
            view.clear();
            match tab.p_view.to_str() {
                Some("root") => {
                    view.add_item("/", PathBuf::from("/"));
                    view.set_selection(0);
                }
                Some(_) | None => {
                    for (index, entry) in App::get_path_iter(&tab.p_view)
                        .filter_entry(|e| e.path().is_dir() && !filter::is_hidden(e))
                        .enumerate()
                    {
                        let entry = entry.unwrap();
                        if entry.path() == &tab.c_view {
                            i = index;
                        }
                        match entry.file_name().to_str() {
                            Some(c) => {
                                view.add_item(format!("  {}", c), PathBuf::from(entry.path()))
                            }
                            None => {}
                        };
                    }
                    for entry in App::get_path_iter(&tab.p_view)
                        .filter_entry(|e| e.path().is_file() && !filter::is_hidden(e))
                    {
                        let entry = entry.unwrap();
                        match entry.file_name().to_str() {
                            Some(c) => {
                                view.add_item(format!("  {}", c), PathBuf::from(entry.path()))
                            }
                            None => {}
                        };
                    }
                    view.set_selection(i);
                }
            }
        });
        // tab.p_focused = i;
        tab.focused.insert(PathBuf::from(&tab.p_view), i);
        debug!("Updated focused for parent: {:?}", tab);
    }

    fn get_path_iter(path: &PathBuf) -> walkdir::IntoIter {
        WalkDir::new(path)
            .max_depth(1)
            .min_depth(1)
            .sort_by(|a, b| compare_os_str(&a.file_name(), &b.file_name()))
            .into_iter()
    }

    #[allow(dead_code)]
    fn add_layout(&mut self) {
        // something
    }

    /// Funtion to handle the event loop.
    ///
    /// Currently does a naive call to `siv.run()`
    pub fn run(&mut self) {
        self.siv.run();
    }
}

/// Funtion to update status bar content and preview widget content on selection change.
/// First the status bar is updated to show relevant permission details and size of file.
/// Then preview is updated to reflect details about the selected entry.
fn update_info(siv: &mut Cursive, entry: &PathBuf) {
    siv.call_on_id("preview", |view: &mut TextView| {
        if !entry.is_dir() {
            let data: Mime = guess_mime_type(entry);
            view.set_content(format!("{}/{}", data.type_(), data.subtype()))
        } else {
            view.set_content("This is a directory!".to_string())
        }
    });
    siv.call_on_id("status", |view: &mut TextView| {
        view.set_content(
            Entry::from(PathBuf::from(entry))
                .permission_string()
                .unwrap(),
        );
    });
}

fn create_console(prefix: &str) -> LinearLayout {
    let prefix_view = TextView::new(prefix);
    let edit_view = EditView::new().filler(" ")
        .on_submit(|s, content| {
            debug!("You entered {}", content);
            s.pop_layer();
        })
        .style(ColorStyle::new(ColorType::from(PaletteColor::Background),
        ColorType::from(PaletteColor::Primary))).with_id("console");
    let layout = LinearLayout::horizontal()
        .child(prefix_view)
        .child(edit_view.full_width());
    layout
}
