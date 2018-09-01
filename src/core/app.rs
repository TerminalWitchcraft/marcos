//! Module contains functions related to core functionalities of the app.

use std::env;
use std::error::Error;
use std::path::PathBuf;

use cursive::{Cursive};
use cursive::views::{Dialog, SelectView, LinearLayout, TextView};
use cursive::traits::{Identifiable, Boxable};

use utils::logger;
use ui::tab::Tab;


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

        // Add 'q' to global callback
        siv.add_global_callback('q', |s| s.quit());

        debug!("Loading theme resource file");
        siv.load_theme_file("assets/style.toml").expect("Cannot find file!");
        Self {
            siv,
            focused_entry: 0,
            focused_tab: 0,
        }
    }

    pub fn add_tab(&mut self, name: &str, path: PathBuf) {
        let tab = Tab::from(name, &path);

        let parent_view: SelectView<PathBuf> = tab.p_widget();
        let current_view: SelectView<PathBuf> = tab.c_widget();

        let mut panes = LinearLayout::horizontal();
        panes.add_child(parent_view.with_id(format!("{tab}/parent", tab=name))
                        .max_width(70)
                        .min_width(40)
                        .full_height());
        panes.add_child(current_view.with_id(format!("{tab}/current", tab=name))
                        .max_width(70)
                        .min_width(40)
                        .full_height());
        panes.add_child(TextView::new("Contents")
                        .with_id(format!("{tab}/contents", tab=name))
                        .full_width());
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

