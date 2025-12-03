mod piece_move;

use crate::board::Board;
use crate::standard::moves::StandardMove;
use crate::standard::piece::ColoredStandardPiece;
use crate::standard::validator::piece_move::is_movement_illegal;
use crate::validator::MoveValidator;

/// The standard move validator, implements the basic chess rules.
/// Implements `MoveValidator` for a regular grid board
pub struct StandardMoveValidator {

}

// A standard game can be played on any board that uses the standard piece set as tokens, and is played using
// standard moves. This move validator implements the ruleset for these games.
impl<B: Board<Token = ColoredStandardPiece>> MoveValidator<B, StandardMove> for StandardMoveValidator {
    fn validate(&self, board: &B, mov: &StandardMove) -> bool {
        // An empty move is not valid, you cannot move an empty square.
        if let Some(piece) = board.at(mov.from()) {
            // You cannot move from a square to the same square
            if mov.from() == mov.to() { return false; }
            
            // Simple check if the movement itself is legal.
            // If it isn't, we can skip any complicated checks.
            if is_movement_illegal(piece, board, mov.from(), mov.to()) {
                return false;
            } else {
                // TODO: We have verified that the basic movement is allowed. Now we need to check everything else.
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::board::mailbox::MailboxBoard;
    use crate::square::Square;
    use crate::standard::piece::{PieceColor, StandardPiece};
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