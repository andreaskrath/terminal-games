use crate::{
    app::{App, AppResult},
    chess::Chess,
    tui::Tui,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui::prelude::Backend;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events<B: Backend>(
    key_event: KeyEvent,
    app: &mut App,
    tui: &mut Tui<B>,
) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Up => {
            app.move_up();
        }
        KeyCode::Down => {
            app.move_down();
        }
        KeyCode::Enter => match app.state.selected().expect("should always have a state") {
            0 => {
                let mut chess_game = Chess::new(tui);
                chess_game.launch_chess()?;
            }
            1 => {}
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
