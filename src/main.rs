mod board; // keeps track of the board
mod utils; // utility functions
mod legalmoves; // legal move generation

use board::{Board, draw_board}; // Import Board struct and PIECE_INDEX_MAP

fn main() {
    let mut board = Board{bitboards: & mut [0;12]};
    board.bitboards[3] = 0b10;
    let chessmove = Move{from:1, to:24, capture: false};
    make_move(&mut board, chessmove);
    draw_board(board);
}
#[allow(dead_code)]
struct Move
{
    from: u8,
    to: u8,
    capture: bool,
}

fn make_move(board: &mut Board, chess_move: Move) {
    let from = chess_move.from;
    let to = chess_move.to;
    if let Some(index) = utils::find_bitboard(board, from) {
        println!("found: {}", index);
        utils::move_piece(&mut board.bitboards[index], to, from);
    }
}
