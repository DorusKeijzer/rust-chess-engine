mod board; // keeps track of the board
mod legalmoves;
mod utils; // utility functions // legal move generation

use crate::{
    board::{Board, State, Turn},
    utils::{draw_bb, find_bitboard, BitIter},
};
use legalmoves::make_move;
use legalmoves::rook_attacks;
use legalmoves::unmake_move;
fn main() {
    let (mut board, mut state) = board::standard_start();
    board.draw();
    legalmoves::perft(&mut board, &mut state, 1, 1, true);
}
