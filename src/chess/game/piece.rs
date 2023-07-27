const BLACK_KING: &str = "♔";
const BLACK_QUEEN: &str = "♕";
const BLACK_ROOK: &str = "♖";
const BLACK_KNIGHT: &str = "♘";
const BLACK_BISHOP: &str = "♗";
const BLACK_PAWN: &str = "♙";

const WHITE_KING: &str = "♚";
const WHITE_QUEEN: &str = "♛";
const WHITE_ROOK: &str = "♜";
const WHITE_KNIGHT: &str = "♞";
const WHITE_BISHOP: &str = "♝";
const WHITE_PAWN: &str = "♟";

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    color: PieceColor,
    variant: PieceType,
}

impl Piece {
    pub fn new(color: PieceColor, variant: PieceType) -> Self {
        Self { color, variant }
    }

    pub fn as_str(&self) -> &str {
        use PieceColor::*;
        use PieceType::*;

        match self.color {
            White => match self.variant {
                Pawn => WHITE_PAWN,
                Bishop => WHITE_BISHOP,
                Knight => WHITE_KNIGHT,
                Rook => WHITE_ROOK,
                Queen => WHITE_QUEEN,
                King => WHITE_KING,
            },
            Black => match self.variant {
                Pawn => BLACK_PAWN,
                Bishop => BLACK_BISHOP,
                Knight => BLACK_KNIGHT,
                Rook => BLACK_ROOK,
                Queen => BLACK_QUEEN,
                King => BLACK_KING,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}
