#[derive(Debug, Clone, Copy)]
pub struct Piece {
    color: PieceColor,
    variant: PieceType,
}

impl Piece {
    pub fn new(color: PieceColor, variant: PieceType) -> Self {
        Self { color, variant }
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
