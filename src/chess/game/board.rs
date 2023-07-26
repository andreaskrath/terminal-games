use tui::{text::Line, widgets::Widget};

use super::pieces::Piece;
const CHESS_BOARD_SIZE: usize = 8;

#[derive(Clone)]
pub struct Board {
    /// Represents the chess board, as a 2D, 8 x 8 array.
    board: [[Option<Piece>; CHESS_BOARD_SIZE]; CHESS_BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[None; CHESS_BOARD_SIZE]; CHESS_BOARD_SIZE],
        }
    }

    fn rows(self) -> Vec<String> {
        vec![
            String::from("  ┌───┬───┬───┬───┬───┬───┬───┬───┐"),
            String::from("8 │ ♖ │ ♘ │ ♗ │ ♕ │ ♔ │ ♗ │ ♘ │ ♖ │"),
            String::from("  ├───┼───┼───┼───┼───┼───┼───┼───┤"),
            String::from("7 │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │"),
            String::from("  ├───┼───┼───┼───┼───┼───┼───┼───┤"),
            String::from("6 │   │   │   │   │   │   │   │   │"),
            String::from("  ├───┼───┼───┼───┼───┼───┼───┼───┤"),
            String::from("5 │   │   │   │   │   │   │   │   │"),
            String::from("  ├───┼───┼───┼───┼───┼───┼───┼───┤"),
            String::from("4 │   │   │   │   │ . │   │   │   │"),
            String::from("  ├───┼───┼───┼───┼───┼───┼───┼───┤"),
            String::from("3 │   │   │   │   │ . │   │   │   │"),
            String::from("  ├───┼───┼───┼───┼───┼───┼───┼───┤"),
            String::from("2 │ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │"),
            String::from("  ├───┼───┼───┼───┼───┼───┼───┼───┤"),
            String::from("1 │ ♜ │ ♞ │ ♝ │ ♛ │ ♚ │ ♝ │ ♞ │ ♜ │"),
            String::from("  └───┴───┴───┴───┴───┴───┴───┴───┘"),
            String::from("    A   B   C   D   E   F   G   H  "),
        ]
    }
}

impl Widget for Board {
    fn render(self, area: tui::prelude::Rect, buf: &mut tui::prelude::Buffer) {
        for (y, row) in (area.top()..area.bottom()).zip(self.rows()) {
            buf.set_line(area.x, y, &Line::from(row), area.width);
        }
    }
}

pub struct Position {
    letter: ChessLetter,
    number: ChessNumber,
}

impl Position {
    fn new(letter: u8, number: u8) -> Self {
        Self {
            letter: letter.into(),
            number: number.into(),
        }
    }
}

enum ChessLetter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl From<u8> for ChessLetter {
    /// The provided `u8` must be within the range 0-7, otherwise the function panics.
    fn from(value: u8) -> Self {
        use ChessLetter::*;
        match value {
            0 => A,
            1 => B,
            2 => C,
            3 => D,
            4 => E,
            5 => F,
            6 => G,
            7 => H,
            _ => panic!("impossible to create a chess letter outside the 0-7 range"),
        }
    }
}

enum ChessNumber {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl From<u8> for ChessNumber {
    /// The provided `u8` must be within the range 0-7, otherwise the function panics.
    fn from(value: u8) -> Self {
        use ChessNumber::*;
        match value {
            0 => One,
            1 => Two,
            2 => Three,
            3 => Four,
            4 => Five,
            5 => Six,
            6 => Seven,
            7 => Eight,
            _ => panic!("impossible to create a chess number outside the 0-7 range"),
        }
    }
}
