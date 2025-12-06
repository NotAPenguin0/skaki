use crate::board::Board;
use crate::square::Square;
use crate::standard::piece::{ColoredStandardPiece, PieceColor, StandardPiece};

pub fn is_pawn_move_illegal(color: PieceColor, board_height: u16, from: Square, to: Square) -> bool {
    // A pawn move can only be one of three things
    // - A pawn capture, e.g., 1 square forward diagonally.
    // - A single step forward
    // - Two steps forward
    // First determine which direction is 'forward' for this color.
    let forward: i16 = if color == PieceColor::White { 1 } else { -1 };

    // If the columns of the from and to squares are different, the only possibility left is a pawn capture
    if from.column != to.column {
        // This move is only legal if the rows differ by one unit in the forward direction,
        // and the columns differ by exactly one in any direction.
        // Since this function should return true if the move is illegal, the condition is inverted.
        return u16::abs_diff(from.column, to.column) != 1|| from.row as i16 + forward != to.row as i16;
    }

    // If the columns are the same, check if the pawn is on its starting location
    // TODO: Maybe have a generic helper on Board for this somewhere, or improve intentionality of this code. I'm not a big fan of this line.
    let start_row = if color == PieceColor::White { 1 } else { board_height - 2 };
    if from.row == start_row {
        // Pawn is in its starting row. Can move at most two steps forward, so the move is illegal
        // if the destination row is not equal to either of those places.
        return from.row as i16 + forward != to.row as i16 && from.row as i16 + 2 * forward != to.row as i16;
    }

    // Pawn is not on its starting location and did not make a capture, it can only move one space forward.
    // Since this function should return true if the move is illegal, the condition is inverted.
    from.row as i16 + forward != to.row as i16
}

pub fn is_knight_move_illegal(from: Square, to: Square) -> bool {
    // Knight moves are very simple. They are only legal if the rows differ by one and the columns by two,
    // or the other way around.
    let row_diff = u16::abs_diff(from.row, to.row);
    let col_diff = u16::abs_diff(from.column, to.column);
    // If neither condition is fulfilled, the move is illegal.
    !(row_diff == 1 && col_diff == 2) && !(row_diff == 2 && col_diff == 1)
}

pub fn is_bishop_move_illegal(from: Square, to: Square) -> bool {
    // Bishops can only move diagonally, so they are only legal if the
    // horizontal distance is the same as the vertical distance
    let row_diff = u16::abs_diff(from.row, to.row);
    let col_diff = u16::abs_diff(from.column, to.column);
    row_diff != col_diff
}

pub fn is_rook_move_illegal(from: Square, to: Square) -> bool {
    // Rooks can only move in a straight line.
    let row_diff = u16::abs_diff(from.row, to.row);
    let col_diff = u16::abs_diff(from.column, to.column);
    // The move is only legal if exactly one of row_diff or col_diff is nonzero
    !((row_diff > 0) ^ (col_diff > 0))
}

pub fn is_queen_move_illegal(from: Square, to: Square) -> bool {
    // A queen can move like a bishop and rook combined, so a queen move is illegal if
    // it is neither a legal bishop move nor a legal rook move.
    is_bishop_move_illegal(from, to) && is_rook_move_illegal(from, to)
}

pub fn is_king_move_illegal(from: Square, to: Square) -> bool {
    // A king can move one square in any direction. This means that the move is illegal if the
    // absolute difference in either row/column is more than one.
    let row_diff = u16::abs_diff(from.row, to.row);
    let col_diff = u16::abs_diff(from.column, to.column);
    row_diff > 1 || col_diff > 1
}

/// Implements basic piece movement. Does not do any checking about
/// the rest of the board. Does not make any verdict about whether the move is actually allowed.
/// This method can only be used to fast-fail things like bishops moving straight, or knights jumping across the board.
pub fn is_movement_illegal<B: Board<Token = ColoredStandardPiece>>(piece: ColoredStandardPiece, board: &B, from: Square, to: Square) -> bool {
    // From and to squares need to be inside the board.
    // If not, this move is definitely illegal
    if !board.valid_square(from) || !board.valid_square(to) {
        return true;
    }

    match piece.piece() {
        StandardPiece::Pawn => {
            is_pawn_move_illegal(piece.color(), board.height(), from, to)
        }
        StandardPiece::Knight => {
            is_knight_move_illegal(from, to)
        }
        StandardPiece::Bishop => {
            is_bishop_move_illegal(from, to)
        }
        StandardPiece::Rook => {
            is_rook_move_illegal(from, to)
        }
        StandardPiece::Queen => {
            is_queen_move_illegal(from, to)
        }
        StandardPiece::King => {
            true
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pawn_illegal_teleport() {
        // Tests that pawns cannot teleport across the board.
        assert!(is_pawn_move_illegal(PieceColor::White, 8, Square::parse("e4").unwrap(), Square::parse("c7").unwrap()))
    }

    #[test]
    fn pawn_illegal_capture() {
        // Tests that pawn captures are properly detected, and rejected if they are illegal

        // Diagonal, but one row too far
        assert!(is_pawn_move_illegal(PieceColor::White, 8, Square::parse("e4").unwrap(), Square::parse("f6").unwrap()));
        // Diagonal, but backward
        assert!(is_pawn_move_illegal(PieceColor::Black, 8, Square::parse("e4").unwrap(), Square::parse("d5").unwrap()));
        // Legal capture for white
        assert!(!is_pawn_move_illegal(PieceColor::White, 8, Square::parse("b2").unwrap(), Square::parse("a3").unwrap()));
        // Legal capture for black
        assert!(!is_pawn_move_illegal(PieceColor::Black, 8, Square::parse("f6").unwrap(), Square::parse("g5").unwrap()));
    }

    #[test]
    fn pawn_on_starting_square() {
        // Check non-capturing pawn moves when the pawn is on its starting square

        assert!(is_pawn_move_illegal(PieceColor::White, 8, Square::parse("a2").unwrap(), Square::parse("b2").unwrap()));
        assert!(is_pawn_move_illegal(PieceColor::White, 8, Square::parse("a2").unwrap(), Square::parse("a5").unwrap()));
        // Legal pawn moves for white
        assert!(!is_pawn_move_illegal(PieceColor::White, 8, Square::parse("g2").unwrap(), Square::parse("g3").unwrap()));
        assert!(!is_pawn_move_illegal(PieceColor::White, 8, Square::parse("e2").unwrap(), Square::parse("e4").unwrap()));
        // Legal pawn moves for black
        assert!(!is_pawn_move_illegal(PieceColor::Black, 8, Square::parse("c7").unwrap(), Square::parse("c5").unwrap()));
        assert!(!is_pawn_move_illegal(PieceColor::Black, 8, Square::parse("h7").unwrap(), Square::parse("h6").unwrap()));
    }

    #[test]
    fn pawn_regular_moves() {
        // Check non-capturing pawn moves when the pawn is not on its starting square

        // Cannot move backward
        assert!(is_pawn_move_illegal(PieceColor::White, 8, Square::parse("f4").unwrap(), Square::parse("f3").unwrap()));
        assert!(is_pawn_move_illegal(PieceColor::Black, 8, Square::parse("d5").unwrap(), Square::parse("d6").unwrap()));

        // Can move forward
        assert!(!is_pawn_move_illegal(PieceColor::White, 8, Square::parse("d5").unwrap(), Square::parse("d6").unwrap()));
        assert!(!is_pawn_move_illegal(PieceColor::Black, 8, Square::parse("f4").unwrap(), Square::parse("f3").unwrap()));
    }

    #[test]
    fn test_knight_moves() {
        assert!(is_knight_move_illegal(Square::parse("g1").unwrap(), Square::parse("g2").unwrap()));
        assert!(is_knight_move_illegal(Square::parse("e4").unwrap(), Square::parse("d7").unwrap()));
        assert!(!is_knight_move_illegal(Square::parse("c2").unwrap(), Square::parse("d4").unwrap()));
        assert!(!is_knight_move_illegal(Square::parse("e5").unwrap(), Square::parse("f3").unwrap()));
        assert!(!is_knight_move_illegal(Square::parse("a4").unwrap(), Square::parse("c5").unwrap()));
    }

    #[test]
    fn test_bishop_moves() {
        assert!(is_bishop_move_illegal(Square::parse("f1").unwrap(), Square::parse("f2").unwrap()));
        assert!(is_bishop_move_illegal(Square::parse("e4").unwrap(), Square::parse("c3").unwrap()));
        assert!(!is_bishop_move_illegal(Square::parse("g7").unwrap(), Square::parse("a1").unwrap()));
        assert!(!is_bishop_move_illegal(Square::parse("c6").unwrap(), Square::parse("e8").unwrap()));
    }

    #[test]
    fn test_rook_moves() {
        assert!(is_rook_move_illegal(Square::parse("f1").unwrap(), Square::parse("d7").unwrap()));
        assert!(is_rook_move_illegal(Square::parse("g7").unwrap(), Square::parse("e5").unwrap()));
        assert!(!is_rook_move_illegal(Square::parse("e3").unwrap(), Square::parse("a3").unwrap()));
        assert!(!is_rook_move_illegal(Square::parse("d5").unwrap(), Square::parse("d1").unwrap()));
    }

    #[test]
    fn test_queen_moves() {
        assert!(is_queen_move_illegal(Square::parse("c2").unwrap(), Square::parse("d4").unwrap()));
        assert!(is_queen_move_illegal(Square::parse("e5").unwrap(), Square::parse("f3").unwrap()));
        assert!(is_queen_move_illegal(Square::parse("a4").unwrap(), Square::parse("c5").unwrap()));
        assert!(!is_queen_move_illegal(Square::parse("g7").unwrap(), Square::parse("a1").unwrap()));
        assert!(!is_queen_move_illegal(Square::parse("c6").unwrap(), Square::parse("e8").unwrap()));
        assert!(!is_queen_move_illegal(Square::parse("e3").unwrap(), Square::parse("a3").unwrap()));
        assert!(!is_queen_move_illegal(Square::parse("d5").unwrap(), Square::parse("d1").unwrap()));
    }
    
    #[test]
    fn test_king_moves() {
        assert!(is_king_move_illegal(Square::parse("e1").unwrap(), Square::parse("e4").unwrap()));
        assert!(is_king_move_illegal(Square::parse("c4").unwrap(), Square::parse("f7").unwrap()));
        assert!(is_king_move_illegal(Square::parse("g7").unwrap(), Square::parse("e5").unwrap()));
        assert!(!is_king_move_illegal(Square::parse("a4").unwrap(), Square::parse("b5").unwrap()));
        assert!(!is_king_move_illegal(Square::parse("c2").unwrap(), Square::parse("d1").unwrap()));
        assert!(!is_king_move_illegal(Square::parse("h7").unwrap(), Square::parse("g6").unwrap()));
    }
}