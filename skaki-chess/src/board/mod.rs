use crate::square::Square;

pub mod mailbox;

/// A generic representation of a chess board.
/// A board is any rectangular arrangement of squares, with tokens places on certain squares, and others being empty.
pub trait Board {
    /// The type of token on the board. This can be a regular chess piece, or some special
    /// token (such as the duck in duck chess)
    type Token;

    /// The width of the board
    fn width(&self) -> u16;

    /// The height of the board
    fn height(&self) -> u16;

    /// Whether this square is a valid square on the board.
    fn valid_square(&self, square: Square) -> bool;

    /// Get the token at the specified location.
    ///
    /// Always returns `None` if the square is outside the board.
    fn at(&self, square: Square) -> Option<Self::Token>;

    /// Set the token at the specified location. Set to `None` to clear the square instead.
    ///
    /// Returns Err(_) if the square is outside the board.
    fn set(&mut self, square: Square, token: Option<Self::Token>) -> anyhow::Result<()>;

    /// Clears the entire board, setting each square to the empty token.
    fn clear(&mut self);

    /// Executes a move on the board. If there is a token on the starting square, set the token of the destination square
    /// to this token. Otherwise, do nothing.
    /// Does not check the legality of moves in any way.
    fn make_move(&mut self, from: Square, to: Square) -> anyhow::Result<()>;
}
