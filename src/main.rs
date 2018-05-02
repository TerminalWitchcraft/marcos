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


struct App{
    size: Rect,
    selected: usize,
    item_len: usize
}

impl App {
    pub fn new() -> App {
        App {
            size: Rect::default(),
            selected: 0,
            item_len: 0,
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
    draw(&mut terminal, &mut app);

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
                },
                event::Key::Up => if app.selected > 0 {
                    app.selected -= 1;
                } else {
                    app.selected = app.item_len - 1;
                },
                event::Key::Char('k') => if app.selected > 0 {
                    app.selected -= 1;
                } else {
                    app.selected = app.item_len - 1;
                },
                event::Key::Down => {
                    app.selected += 1;
                    if app.selected > app.item_len - 1 {
                        app.selected = 0;
                    }
                },
                event::Key::Char('j') => {
                    app.selected += 1;
                    if app.selected > app.item_len - 1 {
                        app.selected = 0;
                    }
                },
                _ => {}
            },
        }
        draw(&mut terminal, &mut app);
    }

}

fn draw(t: &mut Terminal<MouseBackend>, app: &mut App) {
    let style = Style::default().fg(Color::White).bg(Color::Black);
    let paths = read_dir("./").unwrap();
    let current_list = paths.into_iter().map(|e| {
        let dir = e.unwrap();
        let p = dir.path();
        p
    }).collect::<Vec<_>>();
    let current_list = current_list.iter().map(|e| {
        match e.to_str() {
            Some(data) => data,
            None => "",
        }
    }).collect::<Vec<_>>();
    app.item_len = current_list.len();
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&[Size::Percent(20), Size::Percent(40), Size::Percent(40)])
        .render(t, &app.size, |t, chunks| {
            SelectableList::default()
                .block(Block::default().title("Previous").borders(Borders::ALL))
                //.items(&app.items)
                .items(&current_list)
                .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
                .select(app.selected)
                .render(t, &chunks[0]);
            SelectableList::default()
                .block(Block::default().title("Previous").borders(Borders::ALL))
                //.items(&app.items)
                .items(&current_list)
                .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
                .select(app.selected)
                .render(t, &chunks[1]);
            SelectableList::default()
                .block(Block::default().title("Previous").borders(Borders::ALL))
                //.items(&app.items)
                .items(&current_list)
                .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
                .select(app.selected)
                .render(t, &chunks[2]);
        });
    t.draw().unwrap();
}
