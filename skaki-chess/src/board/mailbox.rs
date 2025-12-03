use anyhow::{bail, ensure, Context};
use crate::board::Board;
use crate::square::Square;

/// A simple array-based board, storing tokens in a list with each element corresponding to a token (or no token).
pub struct MailboxBoard<T> {
    width: u16,
    height: u16,
    tokens: Vec<Option<T>>,
}

impl<T: Clone> MailboxBoard<T> {
    fn index(&self, row: u16, column: u16) -> usize {
        (row * self.width + column) as usize
    }

    /// Creates a new empty board.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width, height,
            tokens: vec![None; width as usize * height as usize],
        }
    }
}

impl<T: Clone> Board for MailboxBoard<T> {
    type Token = T;

    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn valid_square(&self, square: Square) -> bool {
        let index = self.index(square.row, square.column);
        index < self.tokens.len()
    }

    fn at(&self, square: Square) -> Option<Self::Token> {
        let index = self.index(square.row, square.column);
        self.tokens.get(index)?.clone()
    }

    fn set(&mut self, square: Square, token: Option<Self::Token>) -> anyhow::Result<()> {
        let index = self.index(square.row, square.column);
        if index >= self.tokens.len() {
            bail!("Board index ({}, {}) is out of bounds for board with size ({}, {})", square.row, square.column, self.width(), self.height());
        }
        self.tokens[index] = token;
        Ok(())
    }

    fn clear(&mut self) {
        self.tokens.fill(None);
    }

    fn make_move(&mut self, from: Square, to: Square) -> anyhow::Result<()> {
        ensure!(self.valid_square(from), "Source square is not inside the board");

        if let Some(token) = self.at(from) {
            self.set(to, Some(token))
                .with_context(|| "Destination square is not inside the board".to_string())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board::mailbox::MailboxBoard;
    use crate::square::Square;

    // Anything can be a piece.
    #[derive(Debug, Clone, Eq, PartialEq)]
    enum Piece {
        Bob,
        Alice,
    }

    #[test]
    fn simple_board() {
        let mut board = MailboxBoard::new(4, 4);
        // Size should be correct
        assert_eq!(board.width(), 4);
        assert_eq!(board.height(), 4);

        board.set(Square::new(0, 0), Some(Piece::Bob)).expect("Should be able to set pieces on valid squares.");
        board.set(Square::new(2, 2), Some(Piece::Alice)).expect("Should be able to set pieces on valid squares.");

        // Should be able to retrieve the pieces again
        assert_eq!(board.at(Square::new(0, 0)), Some(Piece::Bob));
        assert_eq!(board.at(Square::new(2, 2)), Some(Piece::Alice));
        // Squares that were not set should be empty
        assert_eq!(board.at(Square::new(2, 1)), None);
    }

    #[test]
    fn out_of_bounds() {
        let mut board = MailboxBoard::<Piece>::new(4, 4);

        assert!(board.at(Square::new(10, 10)).is_none());
        assert!(board.set(Square::new(10, 10), Some(Piece::Alice)).is_err());
    }

    #[test]
    fn clear_board() {
        let mut board = MailboxBoard::new(4, 4);
        board.set(Square::new(0, 0), Some(Piece::Bob)).expect("Should be able to set pieces on valid squares.");
        board.clear();
        assert_eq!(board.at(Square::new(0, 0)), None);
    }
}