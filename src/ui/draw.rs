use ui::tab::MyTab;
use core::app::App;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Widget, SelectableList, Tabs};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Style};


fn draw_tab(t: &mut Terminal<MouseBackend>, tab: &MyTab, selected:usize, area: &Rect) {
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&[Size::Percent(20), Size::Percent(40), Size::Percent(40)])
        .render(t, area, |t, chunks| {
            //Parent View
            SelectableList::default()
            .block(Block::default().title("Previous").borders(Borders::ALL))
            .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
            .select(tab.get_parent_index())
            .items(&tab.get_parent_items())
            .render(t, &chunks[0]);

            //Current View
            SelectableList::default()
            .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
            .block(Block::default().title("Current").borders(Borders::ALL))
            .select(selected)
            .items(&tab.get_current_items())
            .render(t, &chunks[1]);

            //Preview View
            SelectableList::default()
            .block(Block::default().title("Preview").borders(Borders::ALL))
            .items(&tab.get_preview_items())
            .render(t, &chunks[2]);
        });
} // for fn tab_draw

pub fn draw(t: &mut Terminal<MouseBackend>, app: &mut App) {
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
