use crate::app::AppResult;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui::prelude::Backend;

use super::Chess;

pub fn handle_key_events<B: Backend>(
    key_event: KeyEvent,
    chess: &mut Chess<'_, B>,
) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            chess.quit();
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                chess.quit();
            }
        }
        _ => {}
    }
    Ok(())
}
