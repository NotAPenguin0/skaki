use anyhow::ensure;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Square {
    pub row: u16,
    pub column: u16,
}

impl Square {
    pub fn new(row: u16, column: u16) -> Self {
        Self { row, column }
    }

    // Tries to parse a string into square.
    // The string must start with a single letter from a-z denoting the column,
    // followed by a number indicating the row.
    // Note that unlike in code, rows are numbered starting from 1.
    pub fn parse(s: &str) -> anyhow::Result<Self> {
        ensure!(s.len() >= 2, "Valid square string must have at least two characters.");
        let c = s.chars().nth(0).expect("Already ensured that the string has at least two characters.");
        ensure!(c.is_ascii_lowercase(), "First character of square string must be a lowercase ascii character from a-z.");
        // Find column from first character
        let col = c.to_ascii_lowercase() as u16 - 'a' as u16;
        // Find row from the rest of the string. If parsing fails, return an error
        let row: u16 = s[1..].parse()?;
        ensure!(row >= 1, "Square string must have a row number of at least 1");
        // The real row index is one lower, since rows are numbered starting from one in the notation format.
        Ok(Self { row: row - 1, column: col })
    }
}