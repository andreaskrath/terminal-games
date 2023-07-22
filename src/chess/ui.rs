use super::Chess;
use tui::{
    prelude::{
        Alignment, Backend, Constraint,
        Direction::{Horizontal, Vertical},
        Layout,
    },
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub fn render<B: Backend>(chess: &mut Chess<B>, frame: &mut Frame<'_, B>) {
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
        .title("Chess")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(menu_block, vertical_layout[1]);
}
