use crate::board::Board;
use crate::moves::Move;

/// Represents an algorithm to validate whether a move is legal.
/// This can vary from variant to variant, so it is a trait we can swap out
pub trait MoveValidator<B: Board, M: Move> {
    /// Returns true if making the move on the board is legal, according to the rules
    /// of this move validator.
    fn validate(&self, board: &B, mov: &M) -> bool;
}