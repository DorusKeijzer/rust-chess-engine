#![allow(dead_code, unused_parens, unused_variables, unused_imports)]

use crate::{
    board::{self, Board, State, Turn},
    legalmoves,
    utils::{
        self, algebraic_to_square, count_pieces, draw_bb, find_bitboard, square_to_algebraic,
        BitIter,
    },
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

pub fn format_for_debug(mut board: Board, depth: i32) {
    let total = perft(&mut board, depth, depth, false);
    for m in generate_legal_moves(&mut board) {
        let alg_move = alg_move(&m);
        make_move(&mut board, &m, true);
        let perft_score = perft(&mut board, depth - 1, depth - 1, false);
        println!("{} {}", alg_move, perft_score);
        unmake_move(&mut board, &m, true);
    }
    println!("\n{}", total);
}

fn alg_move(chess_move: &Move) -> String {
    let mut from = square_to_algebraic(&chess_move.from).to_owned();
    let mut to = square_to_algebraic(&chess_move.to);
    if chess_move.castled {
        from = match chess_move.from {
            63 => "e1".to_owned(),
            56 => "e1".to_owned(),
            0 => "e8".to_owned(),
            7 => "e8".to_owned(),
            _ => from,
        };
        to = match chess_move.to {
            61 => "g1".to_string(),
            59 => "c1".to_string(),
            5 => "g8".to_string(),
            3 => "c8".to_string(),
            _ => to,
        }
    }
    let promotion = match chess_move.promotion {
        Some(Piece::Rook) => "r".to_string(),
        Some(Piece::Queen) => "q".to_string(),
        Some(Piece::Knight) => "n".to_string(),
        Some(Piece::Bishop) => "b".to_string(),
        _ => "".to_string(),
    };

    from.push_str(&to);
    from.push_str(&promotion);
    from
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

pub fn generate_legal_moves(board: &mut Board) -> Vec<Move> {
    let mut result = Vec::new();
    for &piece in &[
        Piece::Pawn,
        Piece::Rook,
        Piece::Bishop,
        Piece::King,
        Piece::Knight,
        Piece::Queen,
    ] {
        result.extend(legal_moves(board, piece));
    }

    result.extend(castling(board.occupied(), board));
    result
}

fn bitboard_from_piece_and_color(color: &Turn, piece: Piece) -> usize {
    let offset: usize = if color == &Turn::Black { 6 } else { 0 };
    match piece {
        Piece::Pawn => 0 + offset,
        Piece::Rook => 1 + offset,
        Piece::King => 2 + offset,
        Piece::Knight => 3 + offset,
        Piece::Queen => 4 + offset,
        Piece::Bishop => 5 + offset,
    }
}

fn bitboard_from_piece_and_board(board: &Board, piece: Piece) -> usize {
    bitboard_from_piece_and_color(&board.current_state.turn, piece)
}

fn piece_from_square(bb_index: u8) -> Option<Piece> {
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

fn legal_moves(board: &mut Board, piece: Piece) -> Vec<Move> {
    let moves = pseudo_legal_moves(board, piece);
    let mut result = vec![];
    // makes move and if king not in check, push to results
    for chess_move in moves {
        make_move(board, &chess_move, false); // don't update state so we won't run check() for the wrong color
                                              // println!("{} - check: {}", chess_move, check(board));
                                              //println!("draw:");
                                              // board.draw();

        if !check(board) {
            result.push(chess_move)
        }

        unmake_move(board, &chess_move, false); // update_state is false for the same reasons
    }

    return result;
}

pub fn attacks(board: &Board, turn: Turn) -> u64 {
    let (offset, own) = match turn {
        Turn::White => (6, board.all_black()),
        Turn::Black => (0, board.all_white()),
    };

    let occupied = board.occupied();
    let mut attacks = 0;

    // for every piece off the opponent, enumerate the instance on the board and add their attack pattern
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
                Piece::Pawn => pawn_captures(board, bit as usize, true),
                Piece::Rook => rook_attacks(occupied, own, bit as usize),
                Piece::Bishop => bishop_attacks(occupied, own, bit as usize),
                Piece::Knight => knight_square_pseudo_legal(board, bit as usize, true),
                Piece::King => king_square_pseudo_legal(board, bit as usize),
                Piece::Queen => queen_attacks(occupied, own, bit as usize),
            };
        }
    }

    attacks
}

fn check(board: &mut Board) -> bool {
    // king:  blacks king if black to move
    // offset:  white pieces if black to move
    // own: blacks pieces if black to move
    //

    let (king, offset, own) = match board.current_state.turn {
        Turn::Black => (board.bitboards[8], 0, board.all_black()),
        Turn::White => (board.bitboards[2], 6, board.all_white()),
    };
    let own = own - king;

    // occupied should not take into account our own king
    // because occupied is used to calculate ray attacks
    // and we need those to be able to intersect with the king
    let occupied = board.occupied() - king;

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
                Piece::Pawn => pawn_captures(board, bit as usize, true),
                Piece::Rook => rook_attacks(occupied, own, bit as usize),
                Piece::Bishop => bishop_attacks(occupied, own, bit as usize),
                Piece::Knight => knight_square_pseudo_legal(board, bit as usize, true),
                Piece::King => king_square_pseudo_legal(board, bit as usize),
                Piece::Queen => queen_attacks(occupied, own, bit as usize),
            };
        }
    }
    // true if attack patterns intersect the king
    return (king & attacks) != 0;
}

fn pseudo_legal_moves(board: &Board, piece: Piece) -> Vec<Move> {
    let mut result = vec![];
    let bb_index = bitboard_from_piece_and_board(board, piece);

    let occupied_squares = board.occupied();
    let own: u64 = match board.current_state.turn {
        Turn::Black => board.all_black(),
        Turn::White => board.all_white(),
    };

    for square in BitIter(board.bitboards[bb_index]) {
        let legal_moves = match piece {
            Piece::Pawn => {
                pawn_square_pseudo_legal(board, square as usize)
                    | pawn_captures(board, square as usize, false)
            }
            Piece::Rook => rook_attacks(occupied_squares, own, square as usize),
            Piece::Bishop => bishop_attacks(occupied_squares, own, square as usize),
            Piece::Knight => knight_square_pseudo_legal(board, square as usize, false),
            Piece::King => king_square_pseudo_legal(board, square as usize),
            Piece::Queen => queen_attacks(occupied_squares, own, square as usize),
        };
        result.extend(pseudo_legal_to_moves(
            board,
            legal_moves,
            square as u8,
            piece,
        ));
    }

    result
}

fn king_square_pseudo_legal(board: &Board, square: usize) -> u64 {
    match board.current_state.turn {
        Turn::Black => !board.all_black() & KING_MOVES[square],
        Turn::White => !board.all_white() & KING_MOVES[square],
    }
}

/// Computes pseudo-legal moves for a knight on the given square.
///
/// It returns a bitboard representing where the knight could move from the given square,
/// considering other pieces on the board and the turn's color.
fn knight_square_pseudo_legal(board: &Board, square: usize, exclude_king: bool) -> u64 {
    if exclude_king {
        match board.current_state.turn {
            Turn::Black => (!(board.all_black()) | board.bitboards[8]) & KNIGHT_MOVES[square],
            Turn::White => (!(board.all_white()) | board.bitboards[2]) & KNIGHT_MOVES[square],
        }
    } else {
        match board.current_state.turn {
            Turn::Black => !board.all_black() & KNIGHT_MOVES[square],
            Turn::White => !board.all_white() & KNIGHT_MOVES[square],
        }
    }
}

pub fn pawn_captures(board: &Board, square: usize, reverse_state: bool) -> u64 {
    let square = utils::mask(square as u8);
    let occ = board.occupied();
    let mut result: u64 = 0;
    let white_to_play = board.current_state.turn == Turn::White;
    let mut opponent: u64 = if white_to_play ^ reverse_state {
        board.all_black()
    } else {
        board.all_white()
    };

    if white_to_play ^ reverse_state {
        if (square >> 7) & occ != 0 && square & 0x7F7F7F7F7F7F7F7F != 0 {
            result |= square >> 7;
        }
        if (square >> 9) & occ != 0 && square & 0xFEFEFEFEFEFEFEFE != 0 {
            result |= square >> 9;
        }
        if let Some(en_passant_square) = board.current_state.en_passant {
            opponent |= utils::mask(en_passant_square);
            if utils::mask(en_passant_square) & (square >> 7) != 0
                && square & 0x8080808080808080 == 0
            {
                result |= square >> 7;
            }
            if utils::mask(en_passant_square) & (square >> 9) != 0
                && square & 0x0101010101010101 == 0
            {
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
        if let Some(en_passant_square) = board.current_state.en_passant {
            opponent |= utils::mask(en_passant_square);
            if utils::mask(en_passant_square) & (square << 7) != 0
                && square & 0x0101010101010101 == 0
            {
                result |= square << 7;
            }
            if utils::mask(en_passant_square) & (square << 9) != 0
                && square & 0x8080808080808080 == 0
            {
                result |= square << 9;
            }
        }
    }
    return result & opponent;
}

fn pawn_square_pseudo_legal(board: &Board, square: usize) -> u64 {
    let square = utils::mask(square as u8);
    let occ = board.occupied();
    let mut result: u64 = 0;
    let white_to_play = board.current_state.turn == Turn::White;

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

fn bitboard_index_from_square(board: &Board, square: u8) -> Option<u8> {
    for i in 0..12 {
        if board.bitboards[i] & utils::mask(square) != 0 {
            return Some(i as u8);
        }
    }
    return None;
}
/// All pseudo legal, non promoting, non castling moves
fn pseudo_legal_to_moves(board: &Board, bitboard: u64, from_square: u8, piece: Piece) -> Vec<Move> {
    let mut moves = Vec::new();
    let bitboard = bitboard;

    for to_square in BitIter(bitboard) {
        let captured_bb = find_bitboard(board, to_square as u8);
        let mut captured_piece = None;
        let mut en_passant_capture = false;
        if let Some(bb_index) = captured_bb {
            captured_piece = piece_from_square(bb_index as u8);
        }
        if let Some(en_passant_square) = board.current_state.en_passant {
            if piece == Piece::Pawn && en_passant_square == to_square as u8 {
                captured_piece = Some(Piece::Pawn);
                en_passant_capture = true;
            }
        }
        // promotion logic: in the case that a pawn piece is on the opposite row, add all possible promotions to legal moves.
        if piece ==Piece::Pawn &&  // piece must be a pawn
            ((board.current_state.turn == Turn::White && (0..8).contains(&to_square)) || // if white and on the front row
            (board.current_state.turn == Turn::Black && (56..64).contains(&to_square)))
        {
            // if black and on the back row
            for promotion_piece in vec![Piece::Queen, Piece::Bishop, Piece::Rook, Piece::Knight] {
                // add all options for promoting
                {
                    moves.push(Move {
                        from: from_square,
                        to: to_square as u8,
                        piece: piece,
                        promotion: Some(promotion_piece),
                        captured: captured_piece,
                        castled: false,
                        en_passant_capture,
                    })
                }
            }
        } else {
            // all cases other than promoting pawns
            moves.push(Move {
                from: from_square,
                to: to_square as u8,
                piece: piece,
                promotion: None,
                captured: captured_piece,
                castled: false,
                en_passant_capture,
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
    pub promotion: Option<Piece>, // Optional promotion piece
    pub captured: Option<Piece>,  // Optional captured piece
    pub castled: bool, // whether castling happened in this turn (responsible for moving king)
    pub en_passant_capture: bool,
}

impl Move {
    pub fn alg_move(&self) -> String {
        let mut from = square_to_algebraic(&self.from).to_owned();
        let mut to = square_to_algebraic(&self.to);
        if self.castled {
            from = match self.from {
                63 => "e1".to_owned(),
                56 => "e1".to_owned(),
                0 => "e8".to_owned(),
                7 => "e8".to_owned(),
                _ => from,
            };
            to = match self.to {
                61 => "g1".to_string(),
                59 => "c1".to_string(),
                5 => "g8".to_string(),
                3 => "c8".to_string(),
                _ => to,
            }
        }
        let promotion = match self.promotion {
            Some(Piece::Rook) => "R".to_string(),
            Some(Piece::Queen) => "Q".to_string(),
            Some(Piece::Knight) => "N".to_string(),
            Some(Piece::Bishop) => "B".to_string(),
            _ => "".to_string(),
        };

        from.push_str(&to);
        from.push_str(&promotion);
        from
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Piece::Pawn => "P",
                Piece::Rook => "R",
                Piece::Bishop => "B",
                Piece::Knight => "N",
                Piece::King => "K",
                Piece::Queen => "Q",
            }
        )
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let castling = if self.castled { "(Castling) " } else { "" };
        let capture = if let Some(piece) = self.captured {
            "(Capture)"
        } else {
            ""
        };

        // write!(
        //     f,
        //    "{} from {} to {}",
        //    self.piece,
        //     utils::square_to_algebraic(&self.from),
        //     utils::square_to_algebraic(&self.to),
        // )
        write!(
            f,
            "{} from {} to {} {}{}",
            self.piece,
            utils::square_to_algebraic(&self.from),
            utils::square_to_algebraic(&self.to),
            castling,
            capture
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

const WHITE_KING_START: u64 = 0x1000000000000000;
const BLACK_KING_START: u64 = 0x10;
const WHITE_KINGSIDE_ROOK: u64 = 0x8000000000000000;
const WHITE_QUEENSIDE_ROOK: u64 = 0x100000000000000;
const BLACK_KINGSIDE_ROOK: u64 = 0x80;
const BLACK_QUEENSIDE_ROOK: u64 = 0x1;
const WHITE_KINGSIDE_CASTLING_PATH: u64 = 0x6000000000000000;
const WHITE_QUEENSIDE_CASTLING_PATH: u64 = 0xC00000000000000;
const BLACK_KINGSIDE_CASTLING_PATH: u64 = 0x60;
const BLACK_QUEENSIDE_CASTLING_PATH: u64 = 0xc;
/// returns all the currently legal castling moves for the current player
///
/// castling moves are indicated by setting the castled field to `true` in the move struct
/// castling moves only give the movement of the rook. <br>
/// The movement of the king is inferred during make_move()
///
/// It was decided to keep occupied as an argument (rather than
/// evaluating board.occupied() in the method to reduce the number of
/// function calls)
///
pub fn castling(occupied: u64, board: &Board) -> Vec<Move> {
    let mut result = Vec::new();

    let (king_start, rooks, castling_paths, enemy_attacks) = match board.current_state.turn {
        Turn::White => (
            WHITE_KING_START,
            board.bitboards[1],
            [WHITE_KINGSIDE_CASTLING_PATH, WHITE_QUEENSIDE_CASTLING_PATH],
            attacks(board, Turn::White),
        ),
        Turn::Black => (
            BLACK_KING_START,
            board.bitboards[7],
            [BLACK_KINGSIDE_CASTLING_PATH, BLACK_QUEENSIDE_CASTLING_PATH],
            attacks(board, Turn::Black),
        ),
    };

    let king_bitboard = match board.current_state.turn {
        Turn::White => board.bitboards[2],
        Turn::Black => board.bitboards[8],
    };

    if king_bitboard & king_start == 0 {
        return result;
    }

    let castling_moves = [
        (
            board.current_state.can_castle_kingside(),
            castling_paths[0],
            if board.current_state.turn == Turn::White {
                WHITE_KINGSIDE_ROOK
            } else {
                BLACK_KINGSIDE_ROOK
            },
            Move {
                from: if board.current_state.turn == Turn::White {
                    63
                } else {
                    7
                },
                to: if board.current_state.turn == Turn::White {
                    61
                } else {
                    5
                },
                piece: Piece::Rook,
                promotion: None,
                captured: None,
                castled: true,
                en_passant_capture: false,
            },
        ),
        (
            board.current_state.can_castle_queenside(),
            castling_paths[1],
            if board.current_state.turn == Turn::White {
                WHITE_QUEENSIDE_ROOK
            } else {
                BLACK_QUEENSIDE_ROOK
            },
            Move {
                from: if board.current_state.turn == Turn::White {
                    56
                } else {
                    0
                },
                to: if board.current_state.turn == Turn::White {
                    59
                } else {
                    3
                },
                piece: Piece::Rook,
                promotion: None,
                captured: None,
                castled: true,
                en_passant_capture: false,
            },
        ),
    ];

    for (can_castle, path, rook_position, castling_move) in castling_moves.iter() {
        if *can_castle
            && occupied & path == 0
            && rooks & rook_position != 0
            && path & enemy_attacks == 0
            && king_start & enemy_attacks == 0
        {
            result.push(castling_move.clone());
        }
    }
    result
}

pub fn reconstruct_king_move(rook_move: &Move, board: &Board) -> (Move, u8) {
    let mut king_move = Move {
        from: if board.current_state.turn == Turn::White {
            60
        } else {
            4
        }, // Initial square of the king (e1 for White, e8 for Black)
        to: 0,              // Placeholder value
        piece: Piece::King, // King's piece representation
        captured: None,
        promotion: None,
        castled: false, // false allows for recursiive function call in (un)make_move()
        en_passant_capture: false,
    };
    // permission relevant to this castling so we can update the state
    let mut permission_update = 0;

    // Determine kingside or queenside castling based on the rook's move
    if rook_move.from == 0 && rook_move.to == 3 {
        // Black queenside castling
        king_move.to = 2;
        permission_update += 0b0001;
    } else if rook_move.from == 7 && rook_move.to == 5 {
        // Black kingside castling
        king_move.to = 6;
        permission_update += 0b0010;
    } else if rook_move.from == 56 && rook_move.to == 59 {
        // White queenside castling
        king_move.to = 58;
        permission_update += 0b0100;
    } else if rook_move.from == 63 && rook_move.to == 61 {
        // White kingside castling
        king_move.to = 62;
        permission_update += 0b1000;
    } else {
        panic!("Invalid rook move for castling!");
    }
    (king_move, permission_update)
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
    let king_set = king_set | attacks;
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

fn switch_turn(board: &mut Board) -> () {
    board.current_state.turn = if board.current_state.turn == Turn::White {
        Turn::Black
    } else {
        Turn::White
    };
}

fn remove_castling_rights(chess_move: &Move, state: &mut State) {
    if chess_move.piece == Piece::King {
        state.castling_rights &= if state.turn == Turn::White {
            !0b1100
        } else {
            !0b0011
        }; // remove all castling rights for this player
    } else if chess_move.piece == Piece::Rook {
        if state.turn == Turn::White {
            if chess_move.from == 63 {
                state.castling_rights &= !0b1000; // remove kingside castling right for white
            } else if chess_move.from == 56 {
                state.castling_rights &= !0b0100; // remove queenside castling right for white
            }
        } else {
            if chess_move.from == 7 {
                state.castling_rights &= !0b0010; // remove kingside castling right for black
            } else if chess_move.from == 0 {
                state.castling_rights &= !0b0001; // remove queenside castling right for black
            }
        }
    }
}

pub fn unmake_move(board: &mut Board, chess_move: &Move, update_state: bool) {
    if update_state {
        // reverts to the previous state
        // this has to happen first because state is used to revert moves
        if let Some(state) = board.state_history.pop() {
            board.current_state = state;
        }
    }
    if chess_move.castled {
        // unmakes king move in case of castling
        // this call does not update the state to prevent reverting too far
        let (king_move, _) = reconstruct_king_move(chess_move, board);
        unmake_move(board, &king_move, false);
    }

    // Undoes a captured piece
    if let Some(captured_piece) = chess_move.captured {
        if chess_move.en_passant_capture {
            if let Some(ep_square) = board.current_state.en_passant {
                // in case of en passant capture
                if board.current_state.turn == Turn::White {
                    let captured_bb =
                        bitboard_from_piece_and_color(&Turn::Black, chess_move.captured.unwrap());
                    board.bitboards[captured_bb] ^= utils::mask(ep_square) << 8;
                //  move back one row behind e.p. square
                } else {
                    let captured_bb =
                        bitboard_from_piece_and_color(&Turn::White, chess_move.captured.unwrap());
                    board.bitboards[captured_bb] ^= utils::mask(ep_square) >> 8;
                    // move back one row behind e.p. square
                };
            }
        } else {
            let opposite_color = if board.current_state.turn == Turn::White {
                Turn::Black
            } else {
                Turn::White
            };
            let captured_bb =
                bitboard_from_piece_and_color(&opposite_color, chess_move.captured.unwrap());
            board.bitboards[captured_bb as usize] ^= utils::mask(chess_move.to);
        }
    }
    // updates the bitboard of the piece
    let bb_index = bitboard_from_piece_and_board(board, chess_move.piece);

    if chess_move.promotion.is_some() {
        board.bitboards[bb_index] |= utils::mask(chess_move.to); // remove pawn
        let promotion_index = bitboard_from_piece_and_board(board, chess_move.promotion.unwrap());
        board.bitboards[promotion_index] &= !utils::mask(chess_move.to); // add promoted piece
    }
    board.bitboards[bb_index] ^= utils::mask(chess_move.to);
    board.bitboards[bb_index] ^= utils::mask(chess_move.from);
}

// for EP state updates

const BLACK_PAWN_START: [u8; 8] = [8, 9, 10, 11, 12, 13, 14, 15];
const BLACK_PAWN_DOUBLE_MOVE: [u8; 8] = [24, 25, 26, 27, 28, 29, 30, 31];
const WHITE_PAWN_START: [u8; 8] = [48, 49, 50, 51, 52, 53, 54, 55];
const WHITE_PAWN_DOUBLE_MOVE: [u8; 8] = [32, 33, 34, 35, 36, 37, 38, 39];

fn is_double_pawn_move(from: u8, to: u8, color: Turn) -> bool {
    match color {
        Turn::White => WHITE_PAWN_START.contains(&from) && WHITE_PAWN_DOUBLE_MOVE.contains(&to),
        Turn::Black => BLACK_PAWN_START.contains(&from) && BLACK_PAWN_DOUBLE_MOVE.contains(&to),
    }
}

pub fn make_move(board: &mut Board, chess_move: &Move, update_state: bool) {
    let mut new_state = board.current_state.clone();

    if chess_move.castled
    // in case of castling, move the king too
    {
        let (king_move, permission_update) = reconstruct_king_move(chess_move, board);
        make_move(board, &king_move, false); // move king but don't switch sides yet
        new_state.castling_rights &= if new_state.turn == Turn::White {
            !0b1100
        } else {
            !0b0011
        }; // remove all castling rights for this player
    }
    remove_castling_rights(chess_move, &mut new_state);
    let bb_index = bitboard_from_piece_and_board(board, chess_move.piece);

    // if a piece is captured, find the corresponding bitboard and remove the piece there
    if let Some(captured_piece) = chess_move.captured {
        let opposite_color = if board.current_state.turn == Turn::White {
            Turn::Black
        } else {
            Turn::White
        };
        let captured_bb: usize = bitboard_from_piece_and_color(&opposite_color, captured_piece);

        let captured_index = if chess_move.en_passant_capture {
            match board.current_state.en_passant {
                Some(square) => {
                    // in case of en passant capture
                    match board.current_state.turn {
                        Turn::Black => square - 8, // the piece one row before the e.p. square
                        Turn::White => square + 8, // the piece one row behind the e.p. square
                    }
                }
                None => chess_move.to, // find the piece corresponding to the board
            }
        } else {
            chess_move.to
        };

        board.bitboards[captured_bb as usize] ^= utils::mask(captured_index);
    }
    board.bitboards[bb_index] ^= utils::mask(chess_move.to);
    board.bitboards[bb_index] ^= utils::mask(chess_move.from);

    // set or reset EP state
    new_state.en_passant = if chess_move.piece == Piece::Pawn
        && is_double_pawn_move(chess_move.from, chess_move.to, board.current_state.turn)
    {
        Some(((chess_move.from + chess_move.to) / 2) as u8)
    } else {
        None
    };

    if chess_move.promotion.is_some() {
        board.bitboards[bb_index] &= !utils::mask(chess_move.to); // remove pawn

        let promotion_index = bitboard_from_piece_and_board(board, chess_move.promotion.unwrap());
        board.bitboards[promotion_index] |= utils::mask(chess_move.to); // add promoted piece
    }

    if update_state {
        board.state_history.push(board.current_state.clone());
        board.current_state = new_state;
        switch_turn(board);
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
pub fn perft(board: &mut Board, depth: i32, startdepth: i32, verbose: bool) -> i32 {
    if depth == 0 {
        return 1;
    }

    let moves = generate_legal_moves(board);
    let mut num_moves: i32 = 0;
    for m in moves {
        if verbose {
            println!("{}{}", "  ".repeat((startdepth - depth) as usize), m);
        }
        make_move(board, &m, true);
        let newmoves = perft(board, depth - 1, startdepth, verbose);
        if verbose && depth != 1 {
            println!(
                "{}Total for {m}: {}",
                "  ".repeat((startdepth - depth) as usize),
                newmoves,
            );
        }
        num_moves += newmoves;
        unmake_move(board, &m, true)
    }

    return num_moves;
}
