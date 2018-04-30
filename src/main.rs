extern crate tui;
extern crate termion;

use std::io;
use std::thread;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Widget};
use tui::layout::{Direction, Group, Rect, Size};


struct App{
    size: Rect,
}

impl App {
    pub fn new() -> App {
        App {
            size: Rect::default(),
        }
    }
}

enum Event {
    Input(event::Key),
}

fn main() {
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    // thread to listen to keystrokes
    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    // Run the App
    let mut app = App::new();

    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    app.size = terminal.size().unwrap();
    draw(&mut terminal, &app);

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
        }
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => match input {
                event::Key::Char('q') => {
                    terminal.clear().unwrap();
                    terminal.show_cursor().unwrap();
                    break;
                }
                _ => {}
            },
        }
        draw(&mut terminal, &app);
    }

}

fn draw(t: &mut Terminal<MouseBackend>, app: &App) {
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&[Size::Percent(10), Size::Percent(80), Size::Percent(10)])
        .render(t, &app.size, |t, chunks| {
            Block::default()
                .title("Block")
                .borders(Borders::ALL)
                .render(t, &chunks[0]);
            Block::default()
                .title("Block")
                .borders(Borders::ALL)
                .render(t, &chunks[2]);
        });
    t.draw().unwrap();
}
