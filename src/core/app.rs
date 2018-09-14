//! Module contains functions related to core functionalities of the app.

use std::env;
use std::fs;
use std::process;
use std::path::{PathBuf};
use std::collections::HashMap;

use cursive::{Cursive};
use cursive::views::*;
use cursive::traits::{Identifiable, Boxable, Scrollable};
use cursive::event::{Event,EventResult};

use dirs;

use walkdir::WalkDir;
use alphanumeric_sort::compare_os_str;

use mime_guess::guess_mime_type;
use mime_guess::Mime;

use utils::{ logger, filter, info};
use ui::tab::{Tab};
use error::*;


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
    let mut app = App::new()?;
    app.add_tab("1", path)?;
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
    focused_entry: String,
}

impl App {
    /// Create a new instance of cursive with default global callbacks.
    /// `q` is used to quit the cursive instance.
    ///
    /// TODO `:` is used to open the command box
    pub fn new() -> Result<Self> {
        let data_path: PathBuf = dirs::config_dir()
            .ok_or(ErrorKind::DirNotFound{dirname: String::from("CONFIG_DIR")})?;
        let data_path = data_path.join("marcos");
        if !data_path.exists() {
            fs::create_dir_all(&data_path)
                .expect("Cannot create data_dir");
        }
        let asset_file = data_path.join("style.toml");
        if !asset_file.is_file() {
            fs::File::create(&asset_file).expect("Failed to create asset file");
        }
        let mut siv = Cursive::default();
        siv.add_global_callback(Event::CtrlChar('w'), |s| s.quit());

        debug!("Loading theme resource file");
        siv.load_theme_file(asset_file).expect("Cannot find file!");
        Ok(Self {
            siv,
            vec_tabs: HashMap::new(),
            focused_entry: String::new(),
            focused_tab: 0,
        })
    }

    /// [Experimental] Adds a new tab to the main view. Currently only single tab is supported
    /// for the sake of simplicity. Multiple tabs support will land in near future.
    pub fn add_tab(&mut self, name: &str, path: PathBuf) -> Result<()> {
        let tab = Tab::from(name, &path);
        self.vec_tabs.insert(name.to_string(), tab);
        self.focused_entry = name.to_string();

        let current_tab: &Tab = match self.vec_tabs.get(name) {
            Some(x)     => x,
            None        => &self.vec_tabs["1"] // The default tab
        };
        debug!("Creating parent and current widgets");
        let (p_widget, mut c_widget) = Self::get_widget(&current_tab);
        c_widget.set_on_select(change_content);
        debug!("Got number of items in p_widget: {}", p_widget.len());
        debug!("Got number of items in c_widget: {}", c_widget.len());

        let c_widget = OnEventView::new(c_widget)
            .on_pre_event_inner('k', |s| {
                let cb = s.select_up(1);
                Some(EventResult::Consumed(Some(cb)))
            })
            .on_pre_event_inner('j', |s| {
                let cb = s.select_down(1);
                Some(EventResult::Consumed(Some(cb)))
            });
        let preview_widget = TextView::new("Preview");

        let mut panes = LinearLayout::horizontal();
        panes.add_child(Panel::new(p_widget.with_id(format!("{}/parent", name))
                        .full_width()
                        .max_width(30)
                        .full_height()));
        panes.add_child(Panel::new(c_widget.with_id(format!("{}/current", name)).scrollable()
                        .full_width()
                        .max_width(40)
                        .full_height()));
        panes.add_child(Panel::new(preview_widget.with_id("preview")
                        .full_width()
                        .full_height()));
        let mut h_panes = LinearLayout::vertical();
        h_panes.add_child(TextView::new(format!("{} {}", info::user_info(), info::disk_info("/"))).with_id("global/tabs"));
        h_panes.add_child(panes);
        //h_panes.add_child(TextView::new("Status").with_id("global/status"));
        let mut status_bar = HideableView::new(TextView::new("Status")
                                               .with_id("global/status"));
        status_bar.unhide();
        // let mut command_view = HideableView::new(Dialog::new()
        //                                          .content(EditView::new()
        //                                                   .on_submit(|siv, data| {}))
        //                                          );
        // command_view.hide();
        h_panes.add_child(status_bar);
        // h_panes.add_child(command_view.with_id("global/command"));
        self.siv.add_layer(h_panes);
        self.siv.add_global_callback('q', |s| s.quit());
        Ok(())
    }

    /// Returns a tuple of parent_view, which displays parent directory and current_view, which
    /// displays contents of current directory.
    fn get_widget(tab: &Tab) -> (SelectView<PathBuf>, SelectView<PathBuf>) {
        let mut c_widget = SelectView::default();
        debug!("Start of first loop, c_widget");
        for entry in WalkDir::new(&tab.c_view.p_buff)
            .max_depth(1)
            .min_depth(1)
            .sort_by(|a, b| compare_os_str(&a.file_name(), &b.file_name()))
            .into_iter()
            .filter_entry(|e| e.path().is_dir() && !filter::is_hidden(e)) {
                let entry = entry.unwrap();
                debug!("Adding to c_widget: {:?}", entry);
                match entry.file_name().to_str() {
                    Some(c)     => c_widget.add_item(format!(r"  {}", c), PathBuf::from(entry.path())),
                    None        => {},
                };
            }
        debug!("Start of second loop, c_widget");
        for entry in WalkDir::new(&tab.c_view.p_buff)
            .max_depth(1)
            .min_depth(1)
            .sort_by(|a, b| compare_os_str(&a.file_name(), &b.file_name()))
            .into_iter()
            .filter_entry(|e| e.path().is_file() && !filter::is_hidden(e)) {
                let entry = entry.unwrap();
                debug!("Adding to c_widget: {:?}", entry);
                match entry.file_name().to_str() {
                    Some(c)     => c_widget.add_item(format!("  {}", c), PathBuf::from(entry.path())),
                    None        => {},
                };
            }
        c_widget.set_selection(0);
        let mut p_widget = SelectView::default();
        let mut i: usize = 0;
        debug!("Start of first loop with index, p_widget");
        for (index,entry) in WalkDir::new(&tab.p_view.p_buff)
            .max_depth(1)
            .min_depth(1)
            .sort_by(|a, b| compare_os_str(&a.file_name(), &b.file_name()))
            .into_iter()
            .filter_entry(|e| e.path().is_dir() && !filter::is_hidden(e)).enumerate() {
                let entry = entry.unwrap();
                debug!("Adding to p_widget: {:?} ", entry);
                if entry.path() == &tab.c_view.p_buff {
                    i = index;
                }
                match entry.file_name().to_str() {
                    Some(c)     => p_widget.add_item(format!("  {}", c), PathBuf::from(entry.path())),
                    None        => {},
                };
            }
        debug!("Start of second loop, p_widget");
        for entry in WalkDir::new(&tab.p_view.p_buff)
            .max_depth(1)
            .min_depth(1)
            .sort_by(|a, b| compare_os_str(&a.file_name(), &b.file_name()))
            .into_iter()
            .filter_entry(|e| e.path().is_file() && !filter::is_hidden(e)) {
                let entry = entry.unwrap();
                debug!("Adding to p_widget: {:?} ", entry);
                match entry.file_name().to_str() {
                    Some(c)     => p_widget.add_item(format!("  {}", c), PathBuf::from(entry.path())),
                    None        => {},
                };
            }
        debug!("Setting selection of p_widget to id: {}", i);
        p_widget.set_selection(i);
        p_widget.set_enabled(false);
        debug!("Number of items in p_widget: {}", p_widget.len());
        debug!("Number of items in c_widget: {}", c_widget.len());
        (p_widget, c_widget)
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


fn change_content(siv: &mut Cursive, entry: &PathBuf) {
    siv.call_on_id("preview", |view: &mut TextView| {
        if !entry.is_dir() {
            let data: Mime = guess_mime_type(entry);
            view.set_content(format!("{}/{}", data.type_(), data.subtype()))
        } else {
            view.set_content("This is a directory!".to_string())
        }
    });
}
