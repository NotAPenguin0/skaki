/// A piece in a standard game of chess
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum StandardPiece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// The color of a piece.
/// Skaki will not support variants involving more than two colors.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PieceColor {
    White,
    Black
}

/// A piece with an associated color (either white or black)
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ColoredStandardPiece {
    piece: StandardPiece,
    color: PieceColor,
}

impl ColoredStandardPiece {
    pub fn new(piece: StandardPiece, color: PieceColor) -> Self {
        ColoredStandardPiece { piece, color }
    }
}