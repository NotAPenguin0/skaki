use anyhow::bail;
use crate::board::Board;

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

    fn at(&self, row: u16, column: u16) -> Option<Self::Token> {
        let index = self.index(row, column);
        self.tokens.get(index)?.clone()
    }

    fn set(&mut self, row: u16, column: u16, token: Option<Self::Token>) -> anyhow::Result<()> {
        let index = self.index(row, column);
        if index >= self.tokens.len() {
            bail!("Board index ({}, {}) is out of bounds for board with size ({}, {})", row, column, self.width(), self.height());
        }
        self.tokens[index] = token;
        Ok(())
    }

    fn clear(&mut self) {
        self.tokens.fill(None);
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board::mailbox::MailboxBoard;

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

        board.set(0, 0, Some(Piece::Bob)).expect("Should be able to set pieces on valid squares.");
        board.set(2, 2, Some(Piece::Alice)).expect("Should be able to set pieces on valid squares.");

        // Should be able to retrieve the pieces again
        assert_eq!(board.at(0, 0), Some(Piece::Bob));
        assert_eq!(board.at(2, 2), Some(Piece::Alice));
        // Squares that were not set should be empty
        assert_eq!(board.at(2, 1), None);
    }

    #[test]
    fn out_of_bounds() {
        let mut board = MailboxBoard::<Piece>::new(4, 4);

        assert!(board.at(10, 10).is_none());
        assert!(board.set(10, 10, Some(Piece::Alice)).is_err());
    }
    
    #[test]
    fn clear_board() {
        let mut board = MailboxBoard::new(4, 4);
        board.set(0, 0, Some(Piece::Bob)).expect("Should be able to set pieces on valid squares.");
        board.clear();
        assert_eq!(board.at(0, 0), None);
    }
}