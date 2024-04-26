use lazy_static::lazy_static;
use crate::utils;

lazy_static! {
    /// Stores all the possible knight moves. `KNIGHT_MOVES[i]` holds the possible moves for a 
    /// knight at square `i`
    static ref KNIGHT_MOVES: [u64; 64] = init_knight_tables();
}

/// Initializes the knight move lookup table.
///
/// This function initializes a lookup table for knight moves on a chessboard.
/// It computes all possible knight moves for each square on the board and stores them in an array.
///
/// Returns:
/// - An array containing the knight moves for each square on the chessboard.
fn init_knight_tables() -> [u64;64]
{
    let mut res = [0;64];
    for i in 0..63  
    {
        let knight_position = utils::mask(i as u8);
        res[i] = knight_attacks(knight_position)
    }
    res
}

/// Computes possible knight attacks from a given position.
///
/// This function calculates the possible knight attacks from a given position on the chessboard.
/// It returns a bitboard representing the squares that can be attacked by a knight from the given position.
///
/// Parameters:
/// - `knight_bb`: A bitboard representing the position of the knight.
///
/// Returns:
/// - A bitboard representing the squares that can be attacked by a knight from the given position.
fn knight_attacks(knight_bb: u64) -> u64
{
    let l1 = (knight_bb >> 1) & 0x7f7f7f7f7f7f7f7f;
    let l2 = (knight_bb >> 2) & 0x3f3f3f3f3f3f3f3f;
    let r1 = (knight_bb << 1) & 0xfefefefefefefefe;
    let r2 = (knight_bb << 2) & 0xfcfcfcfcfcfcfcfc;
    let h1 = l1 | r1;
    let h2 = l2 | r2;
    (h1<<16) | (h1>>16) | (h2<<8) | (h2>>8) ^ knight_bb
}

/// Performs perft (Performance Test) for a given depth.
///
/// This function computes the number of possible moves for a given depth in the game tree.
/// It is used to validate the correctness of a chess engine's move generation.
///
/// Parameters:
/// - `depth`: The depth of the search tree to perform perft.
///
/// Returns:
/// - The number of possible moves at the specified depth.
fn perft(depth: i32) -> i32
{
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perft_test()
    {
        assert_eq!(perft(1), 20);
        assert_eq!(perft(2), 400);
        assert_eq!(perft(3), 8902);
        assert_eq!(perft(4), 197_281);
        assert_eq!(perft(5), 4_865_609);
    }
}
