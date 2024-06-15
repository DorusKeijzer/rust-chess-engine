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
fn main() {
    let state = State {
        turn: Turn::White,
        castling_rights: 0b1100, // Example: White can castle kingside and queenside
        enpassant: None,  // Example: No en passant square
    };

    let rook_move = Move {
        from: 7,
        to: 5,
        piece: Piece::Rook, // Rook's piece representation
        captured: None,
        castled: true,
    };

    let king_move = legalmoves::reconstruct_king_move(&rook_move, &state);
    println!("King's Move: {}", king_move);
}

