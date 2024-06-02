use crate::{
    board::{self, Board},
    utils::{self, draw_bb},
    Turn,
};
use lazy_static::lazy_static;
use std::ops::Index;
use std::slice::SliceIndex;

lazy_static! {
    /// Stores all the ray attacks.
    ///
    /// * 0 : north-west
    /// * 1 : north
    /// * 2 : north-east
    /// * 3 : east
    /// * 4 : south-east
    /// * 5 : south
    /// * 6 : south-west
    /// * 7 : west
    pub static ref RAY_ATTACKS: [[u64; 64]; 8] = init_ray_attacks();

    /// Stores all the possible knight moves.
    /// `KNIGHT_MOVES[i]` holds the possible moves for a knight at square `i`
    pub static ref KNIGHT_MOVES: [u64; 64] = init_knight_tables();

    /// Stores all the possible rook moves.
    /// `ROOK_MOVES[i]` holds the possible moves for a rook at square `i`
    static ref ROOK_MOVES: [u64; 64] = init_rook_tables();

    /// Stores all the possible bishop moves.
    /// `BISHOP_MOVES[i]` holds the possible moves for a bishop at square `i`
    static ref BISHOP_MOVES: [u64; 64] = init_bishop_tables();

    /// Stores all the possible queen moves.
    /// `QUEEN_MOVES[i]` holds the possible moves for a queen at square `i`
    static ref QUEEN_MOVES: [u64; 64] = init_queen_tables();
}

pub fn init_ray_attacks() -> [[u64; 64]; 8] {
    let mut res: [[u64; 64]; 8] = [[0; 64]; 8];
    res[0] = north_west_rays();
    res[1] = north_rays();
    res[2] = north_east_rays();
    res[3] = east_rays();
    res[4] = south_rays();
    // res[5] = south_west_rays();
    res[6] = west_rays();
    // res[7] = north_west_rays();
    res
}

fn north_west_rays() -> [u64; 64] {
    let mut res: [u64; 64] = [0; 64];
    let north_west_ray: u64 = 0x102040810204000; // diagonal
    for row in (0..64).step_by(8).rev() {
        for col in 0..8 {
            let mut mask: u64 = 0x8080808080808080; // north facing ray, to mask wrapping numbers with

            let diagonal = north_west_ray >> col + row;
            for p in 0..col {
                mask |= mask >> 1;
            }
            res[63 - ((col % 8) + row)] = mask & diagonal;
        }
    }

    res
}
fn east_rays() -> [u64; 64] {
    let mut res = [0; 64];
    for i in 0..64 {
        res[i] = 2 * ((1 << (i | 7)) - (1 << i));
    }
    res
}

fn west_rays() -> [u64; 64] {
    let mut res = [0; 64];
    for i in 0..64 {
        res[i] = ((1 << i) - 1) & !((1 << (i & 56)) - 1);
    }
    res
}

fn north_rays() -> [u64; 64] {
    let mut res: [u64; 64] = [0; 64];
    let mut north = 0x0101010101010100; // north facing ray
    for square in 0..64 {
        res[square] = north;
        north <<= 1; // slide north facing ray left (and upwards upon wrap)
    }
    res
}

fn south_rays() -> [u64; 64] {
    let mut res: [u64; 64] = [0; 64];
    let mut south = 36170086419038336; // south facing ray
    for square in (0..64).rev() {
        println!("{:?}", square);
        res[square] = south;
        south >>= 1; // slide south facing ray left (and down upon wrap)
    }
    res
}

fn north_east_rays() -> [u64; 64] {
    let mut res: [u64; 64] = [0; 64];
    let north_east_ray: u64 = 0x8040201008040200; // diagonal

    for i in 0..64 {
        let mut mask: u64 = 0x0101010101010100; // north facing ray, to mask wrapping numbers with
        let diagonal = north_east_ray << i;
        for _ in 0..i % 8
        // creates a mask to mask any wrapping numbers with
        {
            mask |= mask << 1;
        }

        res[i] = diagonal & !mask;
    }

    res
}

/// Initializes the knight move lookup table.
///
/// This function initializes a lookup table for knight moves on a chessboard.
/// It computes all possible knight moves for each square on the board and stores them in an array.
///
/// Returns:
/// - An array containing the knight moves for each square on the chessboard.
fn init_knight_tables() -> [u64; 64] {
    let mut res: [u64; 64] = [0; 64];
    for i in 0..64 {
        let knight_position = utils::mask(i as u8);
        res[i] = knight_attacks(knight_position)
    }
    res
}

/// Returns a bitboard of all white pieces on the board.
///
/// This function combines the bitboards of white pieces (pawns, knights, bishops, rooks, queens, kings)
/// into a single bitboard and returns it.
fn all_white(board: Board) -> u64 {
    let mut res: u64 = 0;
    for bitboard in &board.bitboards[0..6] {
        res |= bitboard; // ORs together all white boards
    }
    res
}

/// Returns a bitboard of all black pieces on the board.
///
/// This function combines the bitboards of black pieces (pawns, knights, bishops, rooks, queens, kings)
/// into a single bitboard and returns it.
fn all_black(board: Board) -> u64 {
    let mut res: u64 = 0;
    for bitboard in &board.bitboards[6..12] {
        res |= bitboard; // ORs together all black boards
    }
    res
}

/// Computes pseudo-legal moves for a knight on the given square.
///
/// It returns a bitboard representing where the knight could move from the given square,
/// considering other pieces on the board and the turn's color.
fn knight_pseudo_legal(board: Board, turn: Turn, square: usize) -> u64 {
    match turn {
        Turn::Black => !all_black(board) & KNIGHT_MOVES[square],
        Turn::White => !all_white(board) & KNIGHT_MOVES[square],
    }
}

/// Initializes the bishop move lookup table.
///
/// This function initializes a lookup table for bishop moves on a chessboard.
/// It computes all possible bishop moves for each square on the board and stores them in an array.
///
/// Returns:
/// - An array containing the bishop moves for each square on the chessboard.
fn init_bishop_tables() -> [u64; 64] {
    todo!()
}

/// Initializes the queen move lookup table.
///
/// This function initializes a lookup table for queen moves on a chessboard.
/// It computes all possible queen moves for each square on the board and stores them in an array.
///
/// Returns:
/// - An array containing the queen moves for each square on the chessboard.
fn init_queen_tables() -> [u64; 64] {
    todo!()
}

/// Initializes the rook move lookup table.
///
/// This function initializes a lookup table for rook moves on a chessboard.
/// It computes all possible rook moves for each square on the board and stores them in an array.
///
/// Returns:
/// - An array containing the rook moves for each square on the chessboard.
fn init_rook_tables() -> [u64; 64] {
    let mut res: [u64; 64] = [0; 64];
    for i in 0..64
    {
        res[i] = RAY_ATTACKS[1][i] |  RAY_ATTACKS[3][i] |  RAY_ATTACKS[5][i] |  RAY_ATTACKS[7][i]; 
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
fn knight_attacks(knight_bb: u64) -> u64 {
    let l1 = (knight_bb >> 1) & 0x7f7f7f7f7f7f7f7f;
    let l2 = (knight_bb >> 2) & 0x3f3f3f3f3f3f3f3f;
    let r1 = (knight_bb << 1) & 0xfefefefefefefefe;
    let r2 = (knight_bb << 2) & 0xfcfcfcfcfcfcfcfc;
    let h1 = l1 | r1;
    let h2 = l2 | r2;
    (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8)
}

#[allow(dead_code)]
#[allow(unused_variables)]
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
fn perft(depth: i32) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perft_test() {
        assert_eq!(perft(1), 20);
        assert_eq!(perft(2), 400);
        assert_eq!(perft(3), 8902);
        assert_eq!(perft(4), 197_281);
        assert_eq!(perft(5), 4_865_609);
    }
}
