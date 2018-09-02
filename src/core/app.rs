//! Module contains functions related to core functionalities of the app.

use std::env;
use std::io::{Write,stdout};
use std::error::Error;
use std::path::PathBuf;
use std::collections::HashMap;

use cursive::{Cursive};
use cursive::views::*;
use cursive::traits::{Identifiable, Boxable};
use cursive::event::EventResult;

use termion;
use termion::raw::IntoRawMode;

use utils::logger;
use ui::tab::{Tab, View};


/// Create a new instance of marcos with the specified backend.
///
/// It also setups the logger for log events
pub fn init() -> Result<App, Box<Error>> {
    logger::init()?;
    let mut app = App::new();
    app.add_tab("1", env::current_dir()?);
    Ok(app)
}

/// The data structure holding various elements related to `App`.
#[allow(dead_code)]
pub struct App {
    /// The main application, the cursive instance.
    pub siv: Cursive,
    /// The vector of tabs
    pub vec_tabs: HashMap<String, Tab>,
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
    pub fn new() -> Self {
        let mut siv = Cursive::default();
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout,"{}{}",
                // Clear the screen.
                termion::clear::All,
                // Hide the cursor.
                termion::cursor::Hide
            ).unwrap();
        // Add 'q' to global callback
        siv.add_global_callback('q', |s| s.quit());

        debug!("Loading theme resource file");
        siv.load_theme_file("assets/style.toml").expect("Cannot find file!");
        Self {
            siv,
            vec_tabs: HashMap::new(),
            focused_entry: 0,
            focused_tab: 0,
        }
    }

    pub fn add_tab(&mut self, name: &str, path: PathBuf) {
        let tab = Tab::from(name, &path);
        self.vec_tabs.insert(name.to_string(), tab);
        self.focused_entry += 1;

        let current_tab: &Tab = match self.vec_tabs.get(name) {
            Some(x)     => x,
            None        => &self.vec_tabs["1"] // The default tab
        };
        let mut p_widget = Self::get_widget(&current_tab.p_view);
        p_widget.set_enabled(false);

        let c_widget = Self::get_widget(&current_tab.c_view);
        let c_widget = OnEventView::new(c_widget)
            .on_pre_event_inner('k', |s| {
                s.select_up(1);
                Some(EventResult::Consumed(None))
            })
            .on_pre_event_inner('j', |s| {
                s.select_down(1);
                Some(EventResult::Consumed(None))
            });
        let preview_widget = TextView::new("Content");

        let mut panes = LinearLayout::horizontal();
        panes.add_child(Panel::new(p_widget.with_id(format!("{}/parent", name))
                        .max_width(70)
                        .min_width(40)
                        .full_height()));
        panes.add_child(Panel::new(c_widget.with_id(format!("{}/current", name))
                        .max_width(70)
                        .min_width(40)
                        .full_height()));
        panes.add_child(Panel::new(preview_widget.with_id(format!("{}/preview", name))
                        .full_width()));
        let mut h_panes = LinearLayout::vertical();
        h_panes.add_child(TextView::new("Tabs").with_id("global/tabs"));
        h_panes.add_child(panes);
        //h_panes.add_child(TextView::new("Status").with_id("global/status"));
        let mut status_bar = HideableView::new(TextView::new("Status")
                                               .with_id("global/status"));
        status_bar.unhide();
        let mut command_view = HideableView::new(Dialog::new()
                                                 .content(EditView::new()
                                                          .on_submit(|siv, data| {}))
                                                 );
        command_view.hide();
        h_panes.add_child(status_bar);
        h_panes.add_child(command_view.with_id("global/command"));
        self.siv.add_layer(h_panes);
        //self.siv.add_layer(Dialog::around(panes).padding((0,0,0,0)));
        // self.siv.add_global_callback('h', move |s| {
        //     tab.go_back();
        // };
    }

    fn get_widget(view: &View) -> SelectView<PathBuf> {
        let mut widget = SelectView::default();
        for item in view.vec_entries.iter() {
            let label: &str = match item.file_name() {
                Some(name) => match name.to_str() {
                    Some(data) => data,
                    None    => "",
                }
                None => ""
            };
        widget.add_item(label, PathBuf::from(item));
        }
        widget
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

