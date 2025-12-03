use crate::moves::Move;
use crate::square::Square;
use crate::standard::piece::StandardPiece;

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
            from,
            to,
            promotion: Some(promotion),
        }
    }
}

impl Move for StandardMove {}