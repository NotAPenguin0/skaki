pub mod standard;


/// Different variants may have different representations of moves.
/// For example,
/// - In Cylinder Chess, the starting and ending board of a move may be different
/// - In Bughouse Chess, a move may not have a starting square, and instead the player places a piece from his supply on the board
/// - ...
pub trait Move {
    // TODO: Think about the basic API requirements of moves, and add them in here
    // For now, nothing is needed and we can simply leave this empty.
}
