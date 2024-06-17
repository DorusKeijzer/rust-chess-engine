mod board; // keeps track of the board
mod legalmoves;
mod utils; // utility functions // legal move generation

use crate::{
    board::{Board, State, Turn},
    utils::{draw_bb, find_bitboard, BitIter},
    legalmoves::{Move, Piece},
};
use legalmoves::make_move;
use legalmoves::rook_attacks;
use legalmoves::unmake_move;

/// TODO:   1. if non castling move, lose castling rights
///             a. king
///             b. rook
///     	2. double check castling rights
///         3. if castling move is made, reconstruct and make king move
///             a. make_move
///             b. unmake_move
fn main() {
    let mut board = Board::new(Some("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R"));
    let mut state = State {
        turn: Turn::White,
        castling_rights: 0b1100, // Example: White can castle kingside and queenside
        enpassant: None,  // Example: No en passant square
    };

    board.draw();
    let rook_move = Move {
        from: 63,
        to: 61,
        piece: Piece::Rook, // Rook's piece representation
        captured: None,
        castled: true,
    };

    let king_move = legalmoves::reconstruct_king_move(&rook_move, &state);
    println!("King's Move: {}", king_move);
    println!("Performing move {rook_move}");
    make_move(&mut board, &rook_move, &mut state, false);
    println!("Performing move {king_move}");
    make_move(&mut board, &king_move, &mut state, true);
    board.draw();
    }

