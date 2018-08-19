//! Module contains functions related to core functionalities of the app.
use cursive::{self, Cursive};
use termion;

use utils::logger;


/// Create a new instance of marcos with the specified backend.
///
/// It also setups the logger for log events
pub fn init() -> App {
    logger::init().unwrap();
    App::new()
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


    /// Funtion to handle the event loop.
    ///
    /// Currently does a naive call to `siv.run()`
    pub fn run(&mut self) {
        self.siv.run();
    }

}

