use super::{board::Position, pieces::Piece};

pub struct Player {
    moves: Vec<(Piece, Position, Position)>,
    taken_pieces: Vec<Piece>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            moves: Vec::new(),
            taken_pieces: Vec::new(),
        }
    }
}
