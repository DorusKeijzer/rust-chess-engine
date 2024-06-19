use crate::{
    board::{self, Board, State, Turn},
    utils::{self, draw_bb, find_bitboard, square_to_algebraic, BitIter},
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

fn bitboard_from_piece_and_color(color: &Turn, piece: Piece) -> usize
{
    let offset: usize = if color == &Turn::Black {
        6
    } else {
        0
    };
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
        if !check(board, false) {
            result.push(chess_move)
        }

        unmake_move(board, &chess_move, false); // update_state is false for the same reasons
    }

    return result;
}

fn check(board: &mut Board, draw: bool) -> bool {
    // king:  blacks king if black to move
    // offset:  white pieces if black to move
    // own: blacks pieces if black to move
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
                Piece::Knight => knight_square_pseudo_legal(board, bit as usize),
                Piece::King => king_square_pseudo_legal(board, bit as usize),
                Piece::Queen => queen_attacks(occupied, own, bit as usize),
            };
        }
    }
    // true if attack patterns intersect the king
    if draw {
        println!("king");
        draw_bb(king);
        println!("own");
        draw_bb(own);
        println!("attacks");
        draw_bb(attacks);
        println!("king & attacks");
        draw_bb(king & attacks);
    }
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
            Piece::Knight => knight_square_pseudo_legal(board, square as usize),
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
fn knight_square_pseudo_legal(board: &Board, square: usize) -> u64 {
    match board.current_state.turn {
        Turn::Black => !board.all_black() & KNIGHT_MOVES[square],
        Turn::White => !board.all_white() & KNIGHT_MOVES[square],
    }
}

pub fn pawn_captures(board: &Board, square: usize, reverse_state: bool) -> u64 {
    let square = utils::mask(square as u8);
    let occ = board.occupied();
    let mut result: u64 = 0;
    let white_to_play = board.current_state.turn == Turn::White;
    let opponent: u64 = if white_to_play ^ reverse_state {
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
            if en_passant_square as u64 & (square >> 7) != 0 && square & 0x8080808080808080 == 0 {
                result |= square >> 7;
            }
            if en_passant_square as u64 & (square >> 9) != 0 && square & 0x0101010101010101 == 0 {
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
            if en_passant_square as u64 & (square << 7) != 0 && square & 0x0101010101010101 == 0 {
                result |= square << 7;
            }
            if en_passant_square as u64 & (square << 9) != 0 && square & 0x8080808080808080 == 0 {
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

        if let Some(bb_index) = captured_bb {
            captured_piece = piece_from_square(bb_index as u8);
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

        write!(
            f,
            "{} from {} to {}",
            self.piece,
            utils::square_to_algebraic(&self.from),
            utils::square_to_algebraic(&self.to),
        )
        // write!(
        //     f,
        //     "{} from {} to {} {}{}",
        //     self.piece,
        //     utils::square_to_algebraic(self.from),
        //     utils::square_to_algebraic(self.to),
        //     castling,
        //     capture
        // )
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
    let mut result: Vec<Move> = Vec::new();
    if board.current_state.turn == Turn::White {
        // White kingside castling
        if board.current_state.can_castle_kingside() & (occupied & 0x6000000000000000 == 0)
        // no pieces in F1, G1
        {
            result.push(Move {
                from: 63,
                to: 61,
                piece: Piece::Rook, // Rook's piece representation
                promotion: None,
                captured: None,
                castled: true,
            });
        }

        // White queenside castling
        if board.current_state.can_castle_queenside() & (occupied & 0xe00000000000000 == 0)
        // no pieces in B1, C1, D1
        {
            result.push(Move {
                from: 56,
                to: 59,
                piece: Piece::Rook, // Rook's piece representation
                promotion: None,
                captured: None,
                castled: true,
            });
        }
    } else if board.current_state.turn == Turn::Black {
        // Black kingside castling
        if board.current_state.can_castle_kingside() & (occupied & 0x60 == 0)
        // no pieces in F8, G8
        {
            result.push(Move {
                from: 7,
                to: 5,
                piece: Piece::Rook, // Rook's piece representation
                promotion: None,
                captured: None,
                castled: true,
            });
        }

        // Black queenside castling
        if board.current_state.can_castle_queenside() & (occupied & 0xe == 0)
        // no pieces in B8, C8, D8
        {
            result.push(Move {
                from: 0,
                to: 3,
                piece: Piece::Rook, // Rook's piece representation
                promotion: None,
                captured: None,
                castled: true,
            });
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
        let captured_bb = bitboard_from_piece_and_board(board, captured_piece);
        board.bitboards[captured_bb] ^= utils::mask(chess_move.to);
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
    let p = new_state.castling_rights;
    let bb_index = bitboard_from_piece_and_board(board, chess_move.piece);

    // if a piece is captured, find the corresponding bitboard and remove the piece there
    if chess_move.captured.is_some() {
        draw_bb(utils::mask(chess_move.to));
        let captured_bb = bitboard_index_from_square(&board, chess_move.to).unwrap();
        println!("{captured_bb}");
        draw_bb(board.bitboards[captured_bb as usize]);
        board.bitboards[captured_bb as usize] ^= utils::mask(chess_move.to);
    }

    board.bitboards[bb_index] ^= utils::mask(chess_move.to);
    board.bitboards[bb_index] ^= utils::mask(chess_move.from);

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
