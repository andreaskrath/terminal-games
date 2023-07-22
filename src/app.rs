use std::error;
use tui::widgets::ListState;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Signifies whether the application is running.
    pub running: bool,

    /// The currently selected game.
    pub state: ListState,

    /// The games available for selection.
    pub games: Box<[&'static str]>,
}

impl Default for App {
    fn default() -> Self {
        let mut new = Self {
            running: true,
            state: ListState::default(),
            games: Box::new(["Chess", "Minesweeper"]),
        };
        new.state.select(Some(0));
        new
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Changes the selected item, to the one directly above it, in the menu - without wrapping around.
    pub fn move_up(&mut self) {
        let selected = match self.state.selected() {
            Some(val) => {
                if val == 0 {
                    Some(val)
                } else {
                    Some(val - 1)
                }
            }
            None => Some(0),
        };
        self.state.select(selected);
    }

    /// Changes the selected item, to the one directly below it, in the menu - without wrapping around.
    pub fn move_down(&mut self) {
        let selected = match self.state.selected() {
            Some(val) => {
                if val == self.games.len() - 1 {
                    Some(val)
                } else {
                    Some(val + 1)
                }
            }
            None => Some(0),
        };
        self.state.select(selected);
    }
}
