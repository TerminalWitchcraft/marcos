extern crate tui;

use std::io;

use tui::Terminal;
use tui::backend::RawBackend;
use tui::widgets::{Block, Widget, Borders};
use tui::layout::{Group, Direction, Size};


fn main() {
    let mut terminal = init().expect("Failed to initialize");
    draw(&mut terminal).expect("Failed to draw");
}

fn init() -> Result<Terminal<RawBackend>, io::Error> {
    let backend = RawBackend::new()?;
    Terminal::new(backend)
}


fn draw(t: &mut Terminal<RawBackend>) -> Result<(), io::Error> {
    let size = t.size()?;
    Group::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .sizes(&[Size::Percent(20), Size::Percent(35), Size::Percent(35)])
        .render(t, &size, |t, chunks| {
            Block::default()
                .title("Block")
                .borders(Borders::ALL)
                .render(t, &chunks[0]);
            Block::default()
                .title("Block 2")
                .borders(Borders::ALL)
                .render(t, &chunks[1]);
            Block::default()
                .title("Block 3")
                .borders(Borders::ALL)
                .render(t, &chunks[2]);
        });
    t.draw()
}
