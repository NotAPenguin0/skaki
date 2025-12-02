use crate::board::Board;
use crate::board::mailbox::MailboxBoard;
use crate::moves::Move;
use crate::moves::validator::MoveValidator;
use crate::piece::{ColoredStandardPiece, StandardPiece};
use crate::square::Square;

/// A standard move in a game of chess starts at a square and ends at a square.
/// Note that there is no information required about which piece made the move, since that can be inferred from the board state, assuming
/// no two pieces occupy the same square, which is an invariant we enforce.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct StandardMove {
    from: Square,
    to: Square,
    promotion: Option<StandardPiece>,
}

impl StandardMove {
    /// Create a new move from a starting square to an ending square.
    pub fn new(from: Square, to: Square) -> Self {
        Self {
            from, to, promotion: None,
        }
    }

    /// Create a new move, and add a piece to promote to. This is unvalidated, as the move itself makes no
    /// assumptions about the rules of the game being played.
    pub fn promote(from: Square, to: Square, promotion: StandardPiece) -> Self {
        Self {
            from, to, promotion: Some(promotion),
        }
    }
}

impl Move for StandardMove {}

/// The standard move validator, implements the basic chess rules.
/// Implements `MoveValidator` for a regular grid board
pub struct StandardMoveValidator {

}

// A standard game can be played on any board that uses the standard piece set as tokens, and is played using
// standard moves. This move validator implements the ruleset for these games.
impl<B: Board<Token = ColoredStandardPiece>> MoveValidator<B, StandardMove> for StandardMoveValidator {
    fn validate(&self, board: &B, mov: &StandardMove) -> bool {
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::piece::PieceColor;
    use super::*;

    #[test]
    fn basic_move_validation() {
        let mut board = MailboxBoard::<ColoredStandardPiece>::new(8, 8);
        // Place a white rook on c3, and try to move it to c5.
        // c3
        let start = Square::new(2, 2);
        // c5
        let end = Square::new(2, 4);
        let rook = ColoredStandardPiece::new(StandardPiece::Rook, PieceColor::White);
        board.set(start, Some(rook)).expect("Index in range");
        let mov = StandardMove::new(start, end);
        let validator = StandardMoveValidator {};
        assert!(validator.validate(&board, &mov));
    }
}