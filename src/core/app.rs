
use ui::tab::MyTab;
use tui::layout::Rect;


pub struct App <'a>{
    pub size: Rect,
    pub tabs: Vec<MyTab<'a>>,
    pub command: String,
    pub show_command_box: bool,
    pub tabs_title: Vec<&'a str>,
    pub selected_tab: usize,
    pub selected: usize
}

impl<'a> App <'a> {
    pub fn new() -> App <'a> {
        App {
            size: Rect::default(),
            tabs: Vec::new(),
            command: String::new(),
            show_command_box: false,
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

    pub fn run_command(&mut self) {
        println!("Executing: {:?}", self.command.drain(..).collect::<Vec<char>>());
    }
}
