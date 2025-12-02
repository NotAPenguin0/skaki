/// A piece in a game of chess
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Piece {
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
pub struct ColoredPiece {
    piece: Piece,
    color: PieceColor,
}
