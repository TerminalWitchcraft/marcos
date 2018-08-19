//! Module contains functions related to core functionalities of the app.

use std::env;
use std::path::PathBuf;

use cursive::{Cursive};
use cursive::views::{Dialog, SelectView, LinearLayout, TextView};
use cursive::traits::{Identifiable, Boxable};

use utils::logger;
use ui::tab::Tab;


/// Create a new instance of marcos with the specified backend.
///
/// It also setups the logger for log events
pub fn init() -> App {
    logger::init().unwrap();
    let mut app = App::new();
    app.add_tab(String::from("1"), env::current_dir().unwrap());
    app
}

/// The data structure holding various elements related to `App`.
pub struct App {
    /// The main application, the cursive instance.
    pub siv: Cursive,
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
    pub fn new() -> App {
        let mut siv = Cursive::default();

        debug!("Adding 'q' to global callback");
        siv.add_global_callback('q', |s| s.quit());

        debug!("Loading theme resource file");
        siv.load_theme_file("assets/style.toml").unwrap();
        App {
            siv,
            focused_entry: 0,
            focused_tab: 0,
        }
    }

    pub fn add_tab(&mut self, name: String, path: PathBuf) {
        let tab = Tab::from(name, &path);
        let mut parent_view: SelectView = SelectView::default();
        let mut current_view: SelectView = SelectView::default();

        let mut panes = LinearLayout::horizontal();
        panes.add_child(parent_view.with_id("parent").max_width(70).min_width(40).full_height());
        panes.add_child(current_view.with_id("current").max_width(70).min_width(40).full_height());
        panes.add_child(TextView::new("Contents").with_id("contents").full_width());
        self.siv.add_layer(Dialog::around(panes).padding((0,0,0,0)));
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

