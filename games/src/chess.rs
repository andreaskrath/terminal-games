use std::error::Error;

use crossterm::event::{self, Event::Key, KeyCode};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};

pub fn launch_chess<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| render_chess_layout(f))?;

        if let Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }
}

fn render_chess_layout<B: Backend>(f: &mut Frame<B>) {
    let full_screen = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(f.size());
    let full_block = Block::default()
        .title("Chess")
        .title_alignment(tui::layout::Alignment::Center)
        .borders(Borders::ALL)
        .border_type(tui::widgets::BorderType::Rounded);
    f.render_widget(full_block, full_screen[0]);
}
