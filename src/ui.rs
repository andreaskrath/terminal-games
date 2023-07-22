use crate::app::App;
use tui::{
    backend::Backend,
    layout::Alignment,
    prelude::{
        Constraint,
        Direction::{Horizontal, Vertical},
        Layout,
    },
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let center_constraints = [
        Constraint::Percentage(25),
        Constraint::Min(50),
        Constraint::Percentage(25),
    ];

    let horizontal_layout = Layout::default()
        .direction(Horizontal)
        .constraints(center_constraints.as_ref())
        .split(frame.size());

    let vertical_layout = Layout::default()
        .direction(Vertical)
        .constraints(center_constraints.as_ref())
        .split(horizontal_layout[1]);

    let menu_block = Block::default()
        .title("Games Menu")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(menu_block, vertical_layout[1]);

    let list_layout = Layout::default()
        .margin(2)
        .direction(Vertical)
        .constraints([Constraint::Length(0), Constraint::Min(1)].as_ref())
        .split(vertical_layout[1]);

    let items: Vec<ListItem> = app.games.iter().map(|item| ListItem::new(*item)).collect();
    let menu_list = List::new(items)
        .block(Block::default())
        .highlight_style(Style::default().fg(Color::Cyan).bg(Color::White));
    frame.render_stateful_widget(menu_list, list_layout[1], &mut app.state)
}
