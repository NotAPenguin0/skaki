#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Square {
    pub row: u16,
    pub column: u16,
}

impl Square {
    pub fn new(row: u16, column: u16) -> Self {
        Self { row, column }
    }
}