use self::{board::Board, player::Player};

mod board;
mod pieces;
mod player;

pub struct Game {
    board: Board,
    white: Player,
    black: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            white: Player::new(),
            black: Player::new(),
        }
    }
}
