extern crate marcos;
extern crate cursive;
extern crate tui;
extern crate termion;
#[macro_use]extern crate log;


use termion::event;
use tui::Terminal;
use tui::backend::MouseBackend;
use marcos::ui::draw::draw;
use marcos::ui::tab::MyTab;
use marcos::events::input::InputThread;
use marcos::core::app::App;
use marcos::core;

use cursive::Cursive;
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::views::{Dialog, TextView, SelectView};
use cursive::view::{Position, Offset};
use cursive::views::{LinearLayout, DummyView};
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
    let mut siv = Cursive::default();
    let mut panes = LinearLayout::horizontal();
    let picker = file_picker(".");
    panes.add_child(picker);
    panes.add_child(DummyView);
    panes.add_child(TextView::new("Contents"));

    let mut layout = LinearLayout::vertical();
    layout.add_child(panes);
    layout.add_child(DummyView);
    layout.add_child(TextView::new("Status"));
    siv.add_layer(Dialog::around(layout).button("Quit", |a| a.quit()));
    siv.run();
    // //Set up logging
    // core::log::setup_logger().unwrap();

    // let backend = MouseBackend::new().unwrap();
    // let mut terminal = Terminal::new(backend).unwrap();

    // // for keysrokes threads

    // let input_thread = InputThread::new();
    // InputThread::spawn(input_thread.clone_tx());
    // //input_thread.spawn(input_thread.clone_tx());
    // 
    // //Input

    // let mut app = App::new();

    // terminal.clear().unwrap();
    // terminal.hide_cursor().unwrap();
    // app.size = terminal.size().unwrap();
    // //app.tabs.push(MyTab::new());
    // app.add_tab(MyTab::default());
    // draw(&mut terminal, &mut app);

    // //the main loop for ui draw
    // loop {
    //     let size = terminal.size().unwrap();
    //     if size != app.size {
    //         info!("Change in terminal size");
    //         terminal.resize(size).unwrap();
    //         app.size = size;
    //     }

    //     let evt = input_thread.get_evt().unwrap();
    //     match evt {
    //             event::Key::Char('q') => {
    //                 terminal.clear().unwrap();
    //                 terminal.show_cursor().unwrap();
    //                 break;
    //             },
    //             event::Key::Up => if app.selected > 0 {
    //                 app.selected -= 1;
    //             } else {
    //                 app.selected = app.tabs[app.selected_tab].current.count - 1;
    //             },
    //             event::Key::Char('k') => if app.selected > 0 {
    //                 app.selected -= 1;
    //             } else {
    //                 app.selected = app.tabs[app.selected_tab].current.count - 1;
    //             },
    //             event::Key::Down => {
    //                 app.selected += 1;
    //                 if app.selected > app.tabs[app.selected_tab].current.count - 1 {
    //                     app.selected = 0;
    //                 }
    //             },
    //             event::Key::Char('j') => {
    //                 app.selected += 1;
    //                 if app.selected > app.tabs[app.selected_tab].current.count - 1 {
    //                     app.selected = 0;
    //                 }
    //             },
    //             event::Key::Char(':') => {
    //                 // Shows the command box
    //                 app.show_command_box = true;
    //                 // loop through keys until they press enter or esc
    //                 terminal.show_cursor().unwrap();
    //                 loop {
    //                     let in_evt = input_thread.get_evt().unwrap();
    //                     match in_evt {
    //                         event::Key::Esc     => {
    //                             terminal.hide_cursor().unwrap();
    //                             app.show_command_box = false;
    //                             break;
    //                         },
    //                         event::Key::Backspace => {
    //                             app.command.pop();
    //                         },
    //                         event::Key::Char('\n') => {
    //                             app.run_command();
    //                             terminal.hide_cursor().unwrap();
    //                             terminal.clear().unwrap();
    //                             draw(&mut terminal, &mut app);
    //                             app.show_command_box = false;
    //                             break;
    //                         },
    //                         event::Key::Char(c)  => {
    //                             app.command.push(c)
    //                         },
    //                         _   => {},
    //                     }
    //                 }
    //             }
    //             _ => {}

    //     }
    //     draw(&mut terminal, &mut app)
    // }
    // terminal.show_cursor().unwrap();
}
