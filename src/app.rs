use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Signifies whether the application is running.
    pub running: bool,

    /// The index of the currently selected game.
    pub state: u8,

    /// The games available for selection.
    pub games: Box<[&'static str]>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            state: 0,
            games: Box::new(["Chess", "Minesweeper"]),
        }
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
        if self.state > 0 {
            self.state -= 1;
        }
    }

    /// Changes the selected item, to the one directly below it, in the menu - without wrapping around.
    pub fn move_down(&mut self) {
        if self.state < self.games.len() as u8 - 1 {
            self.state += 1;
        }
    }
}
