use crate::{
    board::{self, Board, State, Turn},
    utils::{self, draw_bb, find_bitboard, BitIter},
};
use core::num;
use lazy_static::lazy_static;
use std::ops::Index;
use std::slice::SliceIndex;
use std::{fmt, iter::Enumerate};

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

    /// Stores all the possible knight moves.
    /// `KING_MOVES[i]` holds the possible moves for a knight at square `i`
    pub static ref KING_MOVES: [u64; 64] = init_king_tables();
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
            let mut mask: u64 = 0x0101010101010101; // north facing ray, to mask wrapping numbers with

            let index = col + row;
            let diagonal = north_west_ray << col + row;
            for _ in 0..(col - 1) {
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
        for _ in 0..(i) % 8
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

fn init_king_tables() -> [u64; 64] {
    let mut res: [u64; 64] = [0; 64];
    for i in 0..64 {
        let king_position = utils::mask(i as u8);
        res[i] = king_attacks(king_position)
    }
    res
}

/// Returns a bitboard of all white pieces on the board.
///
/// This function combines the bitboards of white pieces (pawns, knights, bishops, rooks, queens, kings)
/// into a single bitboard and returns it.
pub fn all_white(board: &Board) -> u64 {
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
pub fn all_black(board: &Board) -> u64 {
    let mut res: u64 = 0;
    for bitboard in &board.bitboards[6..12] {
        res |= bitboard; // ORs together all black boards
    }
    res
}

pub fn occupied(board: &Board) -> u64 {
    return all_black(board) | all_white(board);
}

pub fn generate_legal_moves(board: &mut Board, state: &mut State) -> Vec<Move> {
    let mut result = Vec::new();
    for &piece in &[
        Piece::Pawn,
        Piece::Rook,
        Piece::Bishop,
        Piece::King,
        Piece::Knight,
        Piece::Queen,
    ] {
        result.extend(legal_moves(board, state, piece));
    }
    result
}

fn bitboard_from_piece_and_state(state: &State, piece: Piece) -> usize {
    let offset = if state.turn == Turn::Black { 6 } else { 0 };
    match piece {
        Piece::Pawn => 0 + offset,
        Piece::Rook => 1 + offset,
        Piece::King => 2 + offset,
        Piece::Knight => 3 + offset,
        Piece::Queen => 4 + offset,
        Piece::Bishop => 5 + offset,
    }
}

fn piece_from_bitboard_index(bb_index: u8) -> Option<Piece> {
    match bb_index % 6 {
        0 => Some(Piece::Pawn),
        1 => Some(Piece::Rook),
        2 => Some(Piece::King),
        3 => Some(Piece::Knight),
        4 => Some(Piece::Queen),
        5 => Some(Piece::Bishop),
        _ => None,
    }
}

fn legal_moves(board: &mut Board, state: &mut State, piece: Piece) -> Vec<Move> {
    let moves = pseudo_legal_moves(board, state, piece);
    let mut result = vec![];
    // makes move and if king not in check, push to results
    for chess_move in moves {
        make_move(board, &chess_move, state, true);
        if !check(board, state) {
            result.push(chess_move)
        }
        unmake_move(board, &chess_move, state, true);
    }

    return result;
}

fn check(board: &mut Board, state: &State) -> bool {
    // king:  blacks king if black to move
    // offset:  white pieces if black to move
    // own: blacks pieces if black to move
    let (king, offset, own) = match state.turn {
        Turn::Black => (board.bitboards[8], 0, all_black(board)),
        Turn::White => (board.bitboards[2], 6, all_white(board)),
    };
    let own = own - king;

    // occupied should not take into account our own king
    // because occupied is used to calculate ray attacks
    // and we need those to be able to intersect with the king
    let occupied = occupied(board) - king;

    let mut attacks = 0;

    // for every piece, enumerate the instance on the board and add their attack pattern
    for (i, piece) in vec![
        Piece::Pawn,
        Piece::Rook,
        Piece::King,
        Piece::Knight,
        Piece::Queen,
        Piece::Bishop,
    ]
    .iter()
    .enumerate()
    {
        let bb = board.bitboards[i + offset];
        for bit in BitIter(bb) {
            attacks |= match piece {
                Piece::Pawn => pawn_captures(board, state, bit as usize, true),
                Piece::Rook => rook_attacks(occupied, own, bit as usize),
                Piece::Bishop => bishop_attacks(occupied, own, bit as usize),
                Piece::Knight => knight_square_pseudo_legal(board, state, bit as usize),
                Piece::King => king_square_pseudo_legal(board, state, bit as usize),
                Piece::Queen => queen_attacks(occupied, own, bit as usize),
            };
        }
    }
    // true if attack patterns intersect the king
    return (king & attacks) != 0;
}

fn pseudo_legal_moves(board: &Board, state: &State, piece: Piece) -> Vec<Move> {
    let mut result = vec![];
    let bb_index = bitboard_from_piece_and_state(state, piece);

    let occupied_squares = occupied(board);
    let own: u64 = match state.turn {
        Turn::Black => all_black(board),
        Turn::White => all_white(board),
    };

    for square in BitIter(board.bitboards[bb_index]) {
        let legal_moves = match piece {
            Piece::Pawn => {
                pawn_square_pseudo_legal(board, state, square as usize)
                    | pawn_captures(board, state, square as usize, false)
            }
            Piece::Rook => rook_attacks(occupied_squares, own, square as usize),
            Piece::Bishop => bishop_attacks(occupied_squares, own, square as usize),
            Piece::Knight => knight_square_pseudo_legal(board, state, square as usize),
            Piece::King => king_square_pseudo_legal(board, state, square as usize),
            Piece::Queen => queen_attacks(occupied_squares, own, square as usize),
        };

        result.extend(pseudo_legal_to_moves(
            board,
            legal_moves,
            square as u8,
            &state.turn,
            piece,
        ));
    }

    result
}

fn king_square_pseudo_legal(board: &Board, state: &State, square: usize) -> u64 {
    match state.turn {
        Turn::Black => !all_black(board) & KING_MOVES[square],
        Turn::White => !all_white(board) & KING_MOVES[square],
    }
}

/// Computes pseudo-legal moves for a knight on the given square.
///
/// It returns a bitboard representing where the knight could move from the given square,
/// considering other pieces on the board and the turn's color.
fn knight_square_pseudo_legal(board: &Board, state: &State, square: usize) -> u64 {
    match state.turn {
        Turn::Black => !all_black(board) & KNIGHT_MOVES[square],
        Turn::White => !all_white(board) & KNIGHT_MOVES[square],
    }
}

pub fn pawn_captures(board: &Board, state: &State, square: usize, reverse_state: bool) -> u64 {
    let square = utils::mask(square as u8);
    let occ = occupied(board);
    let mut result: u64 = 0;
    let white_to_play = state.turn == Turn::White;
    let opponent: u64 = if white_to_play ^ reverse_state {
        all_black(board)
    } else {
        all_white(board)
    };

    if white_to_play ^ reverse_state {
        if (square >> 7) & occ != 0 && square & 0x7F7F7F7F7F7F7F7F != 0 {
            result |= square >> 7;
        }
        if (square >> 9) & occ != 0 && square & 0xFEFEFEFEFEFEFEFE != 0 {
            result |= square >> 9;
        }
        if let Some(enpassent_square) = state.enpassant {
            if enpassent_square as u64 & (square >> 7) != 0 && square & 0x8080808080808080 == 0 {
                result |= square >> 7;
            }
            if enpassent_square as u64 & (square >> 9) != 0 && square & 0x0101010101010101 == 0 {
                result |= square >> 9;
            }
        }
    } else {
        if (square << 7) & occ != 0 && square & 0xFEFEFEFEFEFEFEFE != 0 {
            result |= square << 7;
        }
        if (square << 9) & occ != 0 && square & 0x7F7F7F7F7F7F7F7F != 0 {
            result |= square << 9;
        }
        if let Some(enpassent_square) = state.enpassant {
            if enpassent_square as u64 & (square << 7) != 0 && square & 0x0101010101010101 == 0 {
                result |= square << 7;
            }
            if enpassent_square as u64 & (square << 9) != 0 && square & 0x8080808080808080 == 0 {
                result |= square << 9;
            }
        }
    }

    return result & opponent;
}

fn pawn_square_pseudo_legal(board: &Board, state: &State, square: usize) -> u64 {
    let square = utils::mask(square as u8);
    let occ = occupied(board);
    let mut result: u64 = 0;
    let white_to_play = state.turn == Turn::White;

    if white_to_play {
        if ((square >> 8) & occ) == 0 {
            result |= square >> 8;
            if square & 0xFF000000000000 != 0 && ((square >> 16) & occ) == 0 {
                result |= square >> 16;
            }
        }
    } else {
        if ((square << 8) & occ) == 0 {
            result |= square << 8;
            if square & 0xFF00 != 0 && ((square << 16) & occ) == 0 {
                result |= square << 16;
            }
        }
    }

    return result;
}

fn bitboard_index_from_square(board: Board, square: u8) -> Option<u8> {
    for i in 0..12 {
        if board.bitboards[i] ^ utils::mask(square) != 0 {
            return Some(i as u8);
        }
    }
    return None;
}

fn pseudo_legal_to_moves(
    board: &Board,
    bitboard: u64,
    from_square: u8,
    turn: &Turn,
    piece: Piece,
) -> Vec<Move> {
    let mut moves = Vec::new();
    let bitboard = bitboard;

    for to_square in BitIter(bitboard) {
        let captured_bb = find_bitboard(board, to_square as u8);
        let mut captured_piece = None;

        if let Some(bb_index) = captured_bb {
            captured_piece = piece_from_bitboard_index(bb_index as u8);
        }
        {
            moves.push(Move {
                from: from_square,
                to: to_square as u8,
                piece: piece,
                captured: captured_piece,
                castled: false,
            })
        }
    }
    moves
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub from: u8, // Source square (0-63)
    pub to: u8,   // Destination square (0-63)
    pub piece: Piece,
    // pub promotion: Option<Piece>, // Optional promotion piece
    pub captured: Option<Piece>, // Optional captured piece
    pub castled: bool, // whether castling happened in this turn (responsible for moving king)
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} from {} to {} ",
            self.piece,
            utils::square_to_algebraic(self.from),
            utils::square_to_algebraic(self.to),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn,
    Rook,
    Bishop,
    Knight,
    King,
    Queen,
}

#[derive(Debug, Copy, Clone)]
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

pub fn bishop_attacks(occupied: u64, own: u64, square: usize) -> u64 {
    return (get_positive_ray_attacks(occupied, Direction::NorthWest, square)
        | get_positive_ray_attacks(occupied, Direction::NorthEast, square)
        | get_negative_ray_attacks(occupied, Direction::SouthWest, square)
        | get_negative_ray_attacks(occupied, Direction::SouthEast, square))
        & !own;
}

pub fn rook_attacks(occupied: u64, own: u64, square: usize) -> u64 {
    return (get_positive_ray_attacks(occupied, Direction::North, square)
        | get_positive_ray_attacks(occupied, Direction::East, square)
        | get_negative_ray_attacks(occupied, Direction::West, square)
        | get_negative_ray_attacks(occupied, Direction::South, square))
        & !own;
}

/// returns all the currently legal castling moves for the current player
pub fn castling(state: &State) -> Vec<Move> {
    let mut result: Vec<Move> = Vec::new();

    if state.turn == Turn::White {
        // White kingside castling
        if state.can_castle_kingside() {
            result.push(Move {
                from: 7,
                to: 5,
                piece: Piece::Rook, // Rook's piece representation
                captured: None,
                castled: true,
            });
        }

        // White queenside castling
        if state.can_castle_queenside() {
            result.push(Move {
                from: 0,
                to: 3,
                piece: Piece::Rook, // Rook's piece representation
                captured: None,
                castled: true,
            });
        }
    } else if state.turn == Turn::Black {
        // Black kingside castling
        if state.can_castle_kingside() {
            result.push(Move {
                from: 63,
                to: 61,
                piece: Piece::Rook, // Rook's piece representation
                captured: None,
                castled: true,
            });
        }

        // Black queenside castling
        if state.can_castle_queenside() {
            result.push(Move {
                from: 56,
                to: 59,
                piece: Piece::Rook, // Rook's piece representation
                captured: None,
                castled: true,
            });
        }
    }

    result
}

pub fn reconstruct_king_move(rook_move: &Move, state: &State) -> Move {
    let mut king_move = Move {
        from: 4,            // Initial square of the king (e1 for White, e8 for Black)
        to: 0,              // Placeholder value
        piece: Piece::King, // King's piece representation
        captured: None,
        castled: true,
    };

    // Determine kingside or queenside castling based on the rook's move
    if rook_move.from == 0 && rook_move.to == 3 {
        // White queenside castling
        king_move.to = 2;
    } else if rook_move.from == 7 && rook_move.to == 5 {
        // White kingside castling
        king_move.to = 6;
    } else if rook_move.from == 56 && rook_move.to == 59 {
        // Black queenside castling
        king_move.from = 60; // Black king starts from e8
        king_move.to = 58;
    } else if rook_move.from == 63 && rook_move.to == 61 {
        // Black kingside castling
        king_move.from = 60; // Black king starts from e8
        king_move.to = 62;
    } else {
        panic!("Invalid rook move for castling!");
    }
    king_move
}

pub fn queen_attacks(occupied: u64, own: u64, square: usize) -> u64 {
    return rook_attacks(occupied, own, square) | bishop_attacks(occupied, own, square);
}

/// northwest to east
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

/// southeast to west
pub fn get_negative_ray_attacks(occupied: u64, dir: Direction, square: usize) -> u64 {
    let attacks = RAY_ATTACKS[dir as usize][square as usize];
    let blocker = attacks & occupied;
    if blocker != 0 {
        let square = 63 - blocker.leading_zeros() as usize;
        attacks ^ RAY_ATTACKS[dir as usize][square as usize]
    } else {
        attacks
    }
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
/// Computes possible king attacks from a given position.
///
/// This function calculates the possible king attacks from a given position on the chessboard.
/// It returns a bitboard representing the squares that can be attacked by a king from the given position.
///
/// Parameters:
/// - `king_bb`: A bitboard representing the position of the king.
///
/// Returns:
/// - A bitboard representing the squares that can be attacked by a king from the given position.
fn king_attacks(king_set: u64) -> u64 {
    let mut attacks = east_one(king_set) | west_one(king_set);
    let mut king_set = king_set | attacks;
    attacks |= north_one(king_set) | south_one(king_set);
    attacks
}

// Placeholder functions for east_one, west_one, north_one, south_one
// These functions need to be implemented according to the specific logic of your application
fn east_one(bitboard: u64) -> u64 {
    bitboard << 1 & !0x0101010101010101 // Example bitmask to handle board boundaries
}

fn west_one(bitboard: u64) -> u64 {
    bitboard >> 1 & !0x8080808080808080 // Example bitmask to handle board boundaries
}

fn north_one(bitboard: u64) -> u64 {
    bitboard << 8
}

fn south_one(bitboard: u64) -> u64 {
    bitboard >> 8
}

fn switch_turn(state: &mut State) -> () {
    state.turn = if state.turn == Turn::White {
        Turn::Black
    } else {
        Turn::White
    };
}

pub fn unmake_move(board: &mut Board, chess_move: &Move, state: &mut State, switch_state: bool) {
    if switch_state {
        switch_turn(state);
    }
    let bb_to_update = &mut board.bitboards;
    let bb_index = bitboard_from_piece_and_state(state, chess_move.piece);

    // undoes a captured piece
    if chess_move.captured.is_some() {
        let captured_bb = bitboard_from_piece_and_state(state, chess_move.captured.unwrap());
        bb_to_update[captured_bb] ^= utils::mask(chess_move.to);
    }
    bb_to_update[bb_index] ^= utils::mask(chess_move.to);
    bb_to_update[bb_index] ^= utils::mask(chess_move.from);
}

pub fn make_move(board: &mut Board, chess_move: &Move, state: &mut State, switch_state: bool) {
    let bb_to_update = &mut board.bitboards;
    let bb_index = bitboard_from_piece_and_state(state, chess_move.piece);

    // if a piece is captured, find the corresponding bitboard and remove the piece there
    if chess_move.captured.is_some() {
        let captured_bb = bitboard_from_piece_and_state(state, chess_move.captured.unwrap());
        bb_to_update[captured_bb] ^= utils::mask(chess_move.to);
    }

    bb_to_update[bb_index] ^= utils::mask(chess_move.to);
    bb_to_update[bb_index] ^= utils::mask(chess_move.from);
    if switch_state {
        switch_turn(state);
    }
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
pub fn perft(
    board: &mut Board,
    state: &mut State,
    depth: i32,
    startdepth: i32,
    verbose: bool,
) -> i32 {
    if depth == 0 {
        return 1;
    }

    let moves = generate_legal_moves(board, state);
    let mut num_moves: i32 = 0;
    for m in moves {
        if verbose {
            println!(
                "{}{} {:?}",
                "  ".repeat((startdepth - depth) as usize),
                m,
                state.turn
            );
        }
        make_move(board, &m, state, true);
        let newmoves = perft(board, state, depth - 1, startdepth, verbose);
        if verbose && depth != 1 {
            println!(
                "{}Total for {m}: {}",
                "  ".repeat((startdepth - depth) as usize),
                newmoves,
            );
        }
        num_moves += newmoves;
        unmake_move(board, &m, state, true)
    }

    return num_moves;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perft_test_1() {
        let mut board: Board = Board::new(Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
        let turn: Turn = Turn::White;
        let mut state: State = State {
            turn,
            castling_rights: 0,
            enpassant: None,
        };
        assert_eq!(perft(&mut board, &mut state, 1, 1, false), 20);
        assert_eq!(perft(&mut board, &mut state, 2, 2, false), 400);
        assert_eq!(perft(&mut board, &mut state, 3, 3, false), 8902);
        assert_eq!(perft(&mut board, &mut state, 4, 4, false), 197_281);
        assert_eq!(perft(&mut board, &mut state, 5, 5, false), 4_865_609);
    }
    #[test]
    fn perft_test_2() {
        let mut board: Board = Board::new(Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
        let turn: Turn = Turn::White;
        let mut state: State = State {
            turn,
            castling_rights: 0,
            enpassant: None,
        };
        assert_eq!(perft(&mut board, &mut state, 1, 1, false), 48);
        assert_eq!(perft(&mut board, &mut state, 2, 2, false), 2039);
        assert_eq!(perft(&mut board, &mut state, 3, 3, false), 97_862);
        assert_eq!(perft(&mut board, &mut state, 4, 4, false), 4_085_603);
        assert_eq!(perft(&mut board, &mut state, 5, 5, false), 193_690_690);
    }
    fn setup_board(fen: &str) -> (Board, State) {
        let board = Board::new(Some(fen));
        let state = State {
            turn: Turn::Black,
            castling_rights: 0,
            enpassant: None,
        };
        (board, state)
    }

    #[test]
    fn test_board_initialization() {
        let (board, state) = setup_board("p7/3p4/8/8/8/8/8/8");
        // Test if the board initializes correctly
        assert_eq!(board.bitboards[6], 2049); // Black pawn
        assert_eq!(state.turn, Turn::Black);
    }

    #[test]
    fn test_legal_moves_for_black_pawn() {
        let (mut board, mut state) = setup_board("8/3p4/8/8/8/8/8/8");
        let moves = generate_legal_moves(&mut board, &mut state);
        // Check that legal moves for black pawns are correct
        let expected_moves = vec![Move {
            from: 11,
            to: 19,
            piece: Piece::Pawn,
            captured: None,
            castled: false,
        }];
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_move_application_and_reversion() {
        let (mut board, mut state) = setup_board("8/3p4/8/8/8/8/8/8");
        let mv = Move {
            from: 11,
            to: 19,
            piece: Piece::Pawn,
            captured: None,
            castled: false,
        };

        board.draw();
        assert_eq!(board.bitboards[6], 2048); // Black pawn restored
        assert_eq!(state.turn, Turn::Black);
        make_move(&mut board, &mv, &mut state, true);
        board.draw();
        // Verify that the move has been made
        assert_eq!(board.bitboards[6], 524288); // Black pawn moved
        assert_eq!(state.turn, Turn::White);

        unmake_move(&mut board, &mv, &mut state, true);
        board.draw();
        // Verify that the move has been undone
        assert_eq!(board.bitboards[6], 2048); // Black pawn restored
        assert_eq!(state.turn, Turn::Black);
    }

    #[test]
    fn test_no_white_pawn_creation() {
        let (mut board, mut state) = setup_board("8/3p4/8/8/8/8/8/8");
        let moves = generate_legal_moves(&mut board, &mut state);

        for mv in &moves {
            make_move(&mut board, &mv, &mut state, true);
            // Ensure no white pawns are created
            assert_eq!(board.bitboards[0], 0); // No white pawns
            unmake_move(&mut board, &mv, &mut state, true);
        }
    }

    #[test]
    fn test_en_passant() {
        let (mut board, mut state) = setup_board("8/8/8/3pP3/8/8/8/8");
        state.turn = Turn::White;
        state.enpassant = Some(27); // The en passant target square is d6 (index 27)

        let legal_moves = generate_legal_moves(&mut board, &mut state);
        let en_passant_move = Move {
            from: 28,
            to: 19,
            piece: Piece::Pawn,
            captured: Some(Piece::Pawn),
            castled: false,
        };
        assert!(legal_moves.contains(&en_passant_move));
    }

    #[test]
    fn test_castling() {
        let (mut board, mut state) = setup_board("r3k2r/8/8/8/8/8/8/R3K2R");
        state.castling_rights = 0b1111; // Both sides can castle both ways

        state.turn = Turn::White;
        let white_castle_moves = generate_legal_moves(&mut board, &mut state);
        assert!(white_castle_moves.iter().any(|m| m.from == 4 && m.to == 6)); // King side castle
        assert!(white_castle_moves.iter().any(|m| m.from == 4 && m.to == 2)); // Queen side castle

        state.turn = Turn::Black;
        let black_castle_moves = generate_legal_moves(&mut board, &mut state);
        assert!(black_castle_moves
            .iter()
            .any(|m| m.from == 60 && m.to == 62)); // King side castle
        assert!(black_castle_moves
            .iter()
            .any(|m| m.from == 60 && m.to == 58)); // Queen side castle
    }

    #[test]
    fn test_pawn_promotion() {
        let (mut board, mut state) = setup_board("8/P7/8/8/8/8/8/8");
        state.turn = Turn::White;

        let legal_moves = generate_legal_moves(&mut board, &mut state);
        let promotion_moves = vec![
            Move {
                from: 8,
                to: 0,
                piece: Piece::Pawn,
                captured: None,
                castled: false,
            }, // Assume it promotes to Queen
        ];
        promotion_moves.iter().for_each(|m| {
            assert!(legal_moves.contains(m), "Missing promotion move: {:?}", m);
        });
        assert_eq!(legal_moves.len(), promotion_moves.len());
    }
}
