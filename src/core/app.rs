
use ui::tab::MyTab;
use tui::widgets::{Block, Borders, Widget, SelectableList, Tabs};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Style};


pub struct App <'a>{
    pub size: Rect,
    pub tabs: Vec<MyTab<'a>>,
    pub tabs_title: Vec<&'a str>,
    pub selected_tab: usize,
    pub selected: usize
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
