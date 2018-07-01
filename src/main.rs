extern crate marcos;
extern crate cursive;
extern crate tui;
extern crate termion;
#[macro_use]extern crate log;


// use termion::event;
// use tui::Terminal;
// use tui::backend::MouseBackend;
// use marcos::ui::draw::draw;
use marcos::ui::tab::MyTab;
// use marcos::events::input::InputThread;
// use marcos::core::app::App;
use marcos::core;

use cursive::Cursive;
use cursive::traits::Boxable;
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::views::{Dialog, TextView, SelectView};
use cursive::views::{LinearLayout, DummyView};
use cursive::align::HAlign;
use cursive::align;

use termion::terminal_size;
use std::fs::{self, DirEntry, File};
use std::io::Read;
use std::path::Path;


fn file_picker<D>(directory: D) -> SelectView<DirEntry>
    where D: AsRef<Path>
{
    let mut view = SelectView::new();
    for entry in fs::read_dir(directory).expect("can't read directory") {
        if let Ok(e) = entry {
            let file_name = e.file_name().into_string().unwrap();
            view.add_item(file_name, e);
        }
    }
    view.on_select(update_status).on_submit(load_contents)
}


fn update_status(app: &mut Cursive, entry: &DirEntry) {
    let mut status_bar = app.find_id::<TextView>("status").unwrap();
    let file_name = entry.file_name().into_string().unwrap();
    let file_size = entry.metadata().unwrap().len();
    let content = format!("{}: {} bytes", file_name, file_size);
    status_bar.set_content(content);
}

fn load_contents(app: &mut Cursive, entry: &DirEntry) {
    let mut text_view = app.find_id::<TextView>("contents").unwrap();
    let content = if entry.metadata().unwrap().is_dir() {
        "<DIR>".to_string()
    } else {
        let mut buf = String::new();
        let _ = File::open(entry.file_name())
            .and_then(|mut f| f.read_to_string(&mut buf))
            .map_err(|e| buf = format!("Error: {}", e));
        buf
    };
    text_view.set_content(content);
}


fn main() {
    core::log::setup_logger().unwrap();

    let default_tab = MyTab::default();
    info!("The terminal size is: {:?}", terminal_size().unwrap());
    let mut current_view = SelectView::default();
    let mut parent_view = SelectView::default();
    for item in default_tab.current.get_entries().iter() {
        current_view.add_item(item.to_string(), 1);
    }
    for item in default_tab.parent.get_entries().iter() {
        parent_view.add_item(item.to_string(), 1);
    }
    parent_view.set_selection(default_tab.get_parent_index());
    parent_view.set_enabled(false);
    let mut siv = Cursive::default();
    let mut panes = LinearLayout::horizontal();
    panes.add_child(parent_view.max_width(70).min_width(40).full_height());
    panes.add_child(current_view.max_width(70).min_width(40).full_height());
    panes.add_child(DummyView);
    panes.add_child(TextView::new("Contents").full_width());
    //info!("Current selction is: {:?}", current_view.selection());

    siv.add_layer(Dialog::around(panes).padding((0,0,0,0))); //.button("Quit", |a| a.quit()));
    siv.add_global_callback('q', |s| s.quit());
    siv.load_theme_file("assets/style.toml").unwrap();
    siv.run();
}
