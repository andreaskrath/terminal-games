use crate::{app::AppResult, event::Event, tui::Tui};
use tui::prelude::Backend;

use self::{game::Game, handler::handle_key_events};

mod game;
mod handler;
mod ui;

pub struct Chess<'a, B: Backend> {
    running: bool,
    tui: Option<&'a mut Tui<B>>,
    pub game: Game,
}

impl<'a, B: Backend> Chess<'a, B> {
    pub fn new(tui: &'a mut Tui<B>) -> Self {
        Self {
            running: true,
            tui: Some(tui),
            game: Game::new(),
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn launch_chess(&mut self) -> AppResult<()> {
        while self.running {
            let tui = self.tui.take().expect("should always contain a tui");
            tui.terminal.draw(|frame| ui::render(self, frame))?;
            match tui.events.next()? {
                Event::Tick => {}
                Event::Key(key_event) => handle_key_events(key_event, self)?,
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
            }
            self.tui = Some(tui);
        }
        Ok(())
    }
}
