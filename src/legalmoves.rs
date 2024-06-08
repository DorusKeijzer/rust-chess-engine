use crate::{
    board::{self, Board},
    utils::{self, draw_bb, BitIter},
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
    pub static ref ROOK_MOVES: [u64; 64] = init_rook_tables();

    /// Stores all the possible bishop moves.
    /// `BISHOP_MOVES[i]` holds the possible moves for a bishop at square `i`
    pub static ref BISHOP_MOVES: [u64; 64] = init_bishop_tables();

    /// Stores all the possible queen moves.
    /// `QUEEN_MOVES[i]` holds the possible moves for a queen at square `i`
    pub static ref QUEEN_MOVES: [u64; 64] = init_queen_tables();
}


pub fn init_ray_attacks() -> [[u64; 64]; 8] {
    let mut res: [[u64; 64]; 8] = [[0; 64]; 8];
    res[Direction::NorthWest as usize] = north_west_rays();
    res[Direction::North as usize] = north_rays();
    res[Direction::NorthEast as usize] = north_east_rays();
    res[Direction::East as usize] = east_rays();
    res[Direction::SouthEast as usize] = south_east_rays();
    res[Direction::South as usize] = south_rays();
    res[Direction::SouthWest as usize] = south_west_rays();
    res[Direction::West as usize] = west_rays();
    res
}

fn north_west_rays() -> [u64; 64] {
    let mut res: [u64; 64] = [0; 64];
    let north_west_ray: u64 = 72624976668147840; // diagonal
    for row in (0..64).step_by(8) {
        for col in (0..8) {
            let mut mask: u64 = 0x0101010101010100; // north facing ray, to mask wrapping numbers with
            let index = col + row;
            let diagonal = north_west_ray << col + row;
            for _ in 0..col {
                mask |= mask << 1;
            }

            res[index as usize] = mask & diagonal;
        }
    }
    res
}

fn south_east_rays() -> [u64; 64] {
    let mut res: [u64; 64] = [0; 64];
    let north_west_ray: u64 = 0x102040810204000; // diagonal
    for row in (0..64).step_by(8).rev() {
        for col in 0..8 {
            let mut mask: u64 = 0x8080808080808080; // north facing ray, to mask wrapping numbers with

            let diagonal = north_west_ray >> col + row;
            for _ in 0..col {
                mask |= mask >> 1;
            }
            res[63 - ((col % 8) + row)] = mask & diagonal;
        }
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

fn south_west_rays() -> [u64; 64] {
    let mut res: [u64; 64] = [0; 64];
    let north_east_ray: u64 = 18049651735527937; // diagonal
    for i in (0..64).rev() {
        let mut mask: u64 = 0x0101010101010101; // north facing ray, to mask wrapping numbers with
        let diagonal = north_east_ray >> 63 - i;
        for _ in 0..i % 8
        // creates a mask to mask any wrapping numbers with
        {
            mask |= mask << 1;
        }
        res[i as usize] = diagonal & mask;
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
        res[square] = south;
        south >>= 1; // slide south facing ray left (and down upon wrap)
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
fn all_white(board: &Board) -> u64 {
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
fn all_black(board: &Board) -> u64 {
    let mut res: u64 = 0;
    for bitboard in &board.bitboards[6..12] {
        res |= bitboard; // ORs together all black boards
    }
    res
}

pub fn occupied(board: &Board) -> u64 {
    return all_black(board) | all_white(board);
}

pub fn generate_legal_moves(board: &Board, turn: &Turn) -> Vec<Move>
{
    return knight_moves(board, turn);

}

fn knight_moves(board:&Board, turn: &Turn) -> Vec<Move>
{
    let mut result = vec![];
    let mut bb = 0; 
    match turn 
    {
        Turn::Black => bb = board.bitboards[8],
        Turn::White => bb = board.bitboards[2],
    }

    for pos in BitIter(bb){
        result.extend(pseudo_legal_to_moves(knight_square_pseudo_legal(board, turn, pos as usize), pos as u8, turn, Piece::Knight))
    }
    println!("{:?}", result);
    return result
}


/// Computes pseudo-legal moves for a knight on the given square.
///
/// It returns a bitboard representing where the knight could move from the given square,
/// considering other pieces on the board and the turn's color.
fn knight_square_pseudo_legal(board: &Board, turn: &Turn, square: usize) -> u64 {
    match turn {
        Turn::Black => !all_black(board) & KNIGHT_MOVES[square],
        Turn::White => !all_white(board) & KNIGHT_MOVES[square],
    }
}

fn pseudo_legal_to_moves(bitboard: u64, from_square: u8, turn: &Turn, piece: Piece) -> Vec<Move>
{
    let mut moves = Vec::new();
    let mut bitboard = bitboard;
    while bitboard != 0
    {
        let to_square = bitboard.trailing_zeros() as u8;
        {
            moves.push(Move{
                    from: from_square, 
                    to: to_square,
                    piece: Some(piece)
                }
            )
        }
        bitboard &= bitboard -1;
    }
    println!{"{:?}", moves}
    moves
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub from: u8,               // Source square (0-63)
    pub to: u8,                 // Destination square (0-63)
    pub piece: Option<Piece>,
    // pub piece: Option<Piece>, // Optional promotion piece
    // pub captured: Option<Piece>, // Optional captured piece
}

#[derive(Debug, Clone, Copy)]
pub enum Piece
{
    Pawn,
    Rook,
    Bishop,
    Knight,
    King, 
    Queen
}


#[derive(Copy, Clone)]
pub enum Direction {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

pub fn get_positive_ray_attacks(occupied: u64, dir: Direction, square: usize) -> u64 {
    let attacks = RAY_ATTACKS[dir as usize][square as usize];
    let blocker = attacks & occupied;
    if blocker != 0 {
        let square = blocker.trailing_zeros();
        attacks ^ RAY_ATTACKS[dir as usize][square as usize]
    } else {
        attacks
    }
}

pub fn get_negative_ray_attacks(occupied: u64, dir: Direction, square: usize) -> u64 {
    unsafe {
        let attacks = RAY_ATTACKS[dir as usize][square as usize];
        let blocker = attacks & occupied;
        if blocker != 0 {
            let square = 63- occupied.leading_zeros();
            attacks ^ RAY_ATTACKS[dir as usize][square as usize]
        } else {
            attacks
        }
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
    let mut res: [u64; 64] = [0; 64];
    for i in 0..64 {
        res[i] = RAY_ATTACKS[0][i] | RAY_ATTACKS[2][i] | RAY_ATTACKS[4][i] | RAY_ATTACKS[6][i];
    }
    res
}

/// Initializes the queen move lookup table.
///
/// This function initializes a lookup table for queen moves on a chessboard.
/// It computes all possible queen moves for each square on the board and stores them in an array.
///
/// Returns:
/// - An array containing the queen moves for each square on the chessboard.
fn init_queen_tables() -> [u64; 64] {
    let mut res: [u64; 64] = [0; 64];
    for i in 0..64 {
        for j in 0..9 {
            res[i] |= RAY_ATTACKS[j][i];
        }
    }

    res
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
    for i in 0..64 {
        res[i] = RAY_ATTACKS[1][i] | RAY_ATTACKS[3][i] | RAY_ATTACKS[5][i] | RAY_ATTACKS[7][i];
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
