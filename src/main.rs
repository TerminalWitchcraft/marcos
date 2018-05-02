#[macro_use]
extern crate tui;
extern crate termion;

use std::io;
use std::thread;
use std::sync::mpsc;
use std::fs::read_dir;
//use std::path::Path;
use std::path::PathBuf;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Widget, SelectableList, Tabs};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Style};

enum Event {
    Input(event::Key),
}

struct App <'a>{
    size: Rect,
    tabs: Vec<MyTab<'a>>,
    tabs_title: Vec<&'a str>,
    selected_tab: usize,
    selected: usize
}

impl<'a> App <'a> {
    pub fn new() -> App <'a> {
        App {
            size: Rect::default(),
            tabs: Vec::new(),
            selected_tab: 0,
            selected: 0,
            tabs_title: Vec::new(),
        }
    }

    pub fn add_tab(&mut self, tab: MyTab<'a>) {
        self.tabs.push(tab);
        if self.selected_tab != 0 {
            self.selected_tab += 1
        }
    }
}

struct MyTab<'b> {
    title: &'b str,
    parent: Vec<PathBuf>,
    current: Vec<PathBuf>,
    preview: Vec<PathBuf>,
}

impl<'b> MyTab <'b> {
    pub fn new(name:&str) -> MyTab {
        MyTab {
            title: name,
            parent: Vec::new(),
            current: Vec::new(),
            preview: Vec::new(),
        }
    }

    pub fn default() -> MyTab <'b> {
        // current directory
        let current_paths = read_dir(".").unwrap();
        let current_list = current_paths.into_iter().map(|e| {
            let dir = e.unwrap();
            let p = dir.path();
            p
        }).collect::<Vec<_>>();

        //parent directory
        let parent_paths = read_dir("..").unwrap();
        let parent_list = parent_paths.into_iter().map(|e| {
            let dir = e.unwrap();
            let p = dir.path();
            p
        }).collect::<Vec<_>>();

        //Populate the MyTab instance
        MyTab {
            title: "Default Void",
            parent: parent_list,
            current: current_list,
            preview: Vec::new(),
        }
    }
}


fn draw_tab(t: &mut Terminal<MouseBackend>, tab: &MyTab, selected:usize, area: &Rect) {
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&[Size::Percent(20), Size::Percent(40), Size::Percent(40)])
        .render(t, area, |t, chunks| {
            //Parent View
            SelectableList::default()
            .block(Block::default().title("Previous").borders(Borders::ALL))
            .items(&tab.parent.iter().map(|e| {
                match e.to_str() {
                    Some(data)  => data,
                    None        => "",
                }
            }).collect::<Vec<_>>())
            .render(t, &chunks[0]);

            //Current View
            SelectableList::default()
            .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
            .block(Block::default().title("Current").borders(Borders::ALL))
            .select(selected)
            .items(&tab.current.iter().map(|e| {
                match e.to_str() {
                    Some(data)  => data,
                    None        => "",
                }
            }).collect::<Vec<_>>())
            .render(t, &chunks[1]);

            //Preview View
            SelectableList::default()
            .block(Block::default().title("Preview").borders(Borders::ALL))
            .items(&tab.current.iter().map(|e| {
                match e.to_str() {
                    Some(data)  => data,
                    None        => "",
                }
            }).collect::<Vec<_>>())
            .render(t, &chunks[2]);
        });
} // for fn tab_draw

fn draw(t: &mut Terminal<MouseBackend>, app: &mut App) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(3), Size::Min(0)])
        .render(t, &app.size, |mut t, chunks|{
            Tabs::default()
                .block(Block::default().borders(Borders::ALL).title("Welcome to marcos"))
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(app.selected_tab)
                .titles(&app.tabs.iter().map(|e| e.title).collect::<Vec<_>>())
                .render(t, &chunks[0]);
            draw_tab(&mut t, &app.tabs[app.selected_tab], app.selected, &chunks[1]);
        });
    t.draw().unwrap();
}

fn main() {
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // for keysrokes threads
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();
    
    //Input
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
                    app.selected = app.tabs[app.selected_tab].current.len() - 1;
                },
                event::Key::Char('k') => if app.selected > 0 {
                    app.selected -= 1;
                } else {
                    app.selected = app.tabs[app.selected_tab].current.len() - 1;
                },
                event::Key::Down => {
                    app.selected += 1;
                    if app.selected > app.tabs[app.selected_tab].current.len() - 1 {
                        app.selected = 0;
                    }
                },
                event::Key::Char('j') => {
                    app.selected += 1;
                    if app.selected > app.tabs[app.selected_tab].current.len() - 1 {
                        app.selected = 0;
                    }
                },
                _ => {}
            },

        }
        draw(&mut terminal, &mut app)
    }
    terminal.show_cursor().unwrap();
}
