extern crate marcos;
extern crate tui;
extern crate termion;


use termion::event;

use tui::Terminal;
use tui::backend::MouseBackend;
use marcos::ui::draw::draw;
use marcos::ui::tab::MyTab;
use marcos::events::input::InputThread;
use marcos::core::app::App;
use marcos::core::log;



fn main() {
    //Set up logging
    log::setup_logger().unwrap();

    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // for keysrokes threads

    let input_thread = InputThread::new();
    InputThread::spawn(input_thread.clone_tx());
    //input_thread.spawn(input_thread.clone_tx());
    
    //Input

    let mut app = App::new();

    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    app.size = terminal.size().unwrap();
    //app.tabs.push(MyTab::new());
    app.add_tab(MyTab::default());
    draw(&mut terminal, &mut app);

    //the main loop for ui draw
    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }

        let evt = input_thread.get_evt().unwrap();
        match evt {
                event::Key::Char('q') => {
                    terminal.clear().unwrap();
                    terminal.show_cursor().unwrap();
                    break;
                },
                event::Key::Up => if app.selected > 0 {
                    app.selected -= 1;
                } else {
                    app.selected = app.tabs[app.selected_tab].current.count - 1;
                },
                event::Key::Char('k') => if app.selected > 0 {
                    app.selected -= 1;
                } else {
                    app.selected = app.tabs[app.selected_tab].current.count - 1;
                },
                event::Key::Down => {
                    app.selected += 1;
                    if app.selected > app.tabs[app.selected_tab].current.count - 1 {
                        app.selected = 0;
                    }
                },
                event::Key::Char('j') => {
                    app.selected += 1;
                    if app.selected > app.tabs[app.selected_tab].current.count - 1 {
                        app.selected = 0;
                    }
                },
                _ => {}

        }
        draw(&mut terminal, &mut app)
    }
    terminal.show_cursor().unwrap();
}
