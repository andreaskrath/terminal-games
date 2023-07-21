use std::error::Error;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event::Key, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run_games_menu(&mut terminal);

    if let Err(err) = result {
        println!("{err}");
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}

fn run_games_menu<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| render_games_menu(f))?;
        if let Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }
}

fn render_games_menu<B: Backend>(f: &mut Frame<B>) {
    let full_screen = Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(f.size());

    let menu_block = Block::default()
        .title("Games Menu")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    f.render_widget(menu_block, full_screen[1]);
}
