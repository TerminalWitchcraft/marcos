extern crate tui;
extern crate termion;

use std::io;
use std::thread;
use std::sync::mpsc;
use std::fs::read_dir;
use std::path::Path;
use std::path::PathBuf;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Widget, SelectableList};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Modifier, Style};


struct App<'a>{
    size: Rect,
    items: Vec<&'a str>
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            size: Rect::default(),
            items: vec!["item1", "item2"],
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
        .sizes(&[Size::Percent(20), Size::Percent(40), Size::Percent(40)])
        .render(t, &app.size, |t, chunks| {
            let style = Style::default().fg(Color::White).bg(Color::Black);
            let paths = read_dir("./").unwrap();
            let items = paths.map(|e| {
                match e {
                    Ok(entry) => entry.path(),
                    _ => PathBuf::new(),
                }
            }).collect::<Vec<_>>();
            SelectableList::default()
                .block(Block::default().title("Previous").borders(Borders::ALL))
                .items(&app.items)
                .render(t, &chunks[0]);
            Block::default()
                .title("Curret")
                .borders(Borders::ALL)
                .render(t, &chunks[1]);
            Block::default()
                .title("Preview")
                .borders(Borders::ALL)
                .render(t, &chunks[2]);
        });
    t.draw().unwrap();
}
