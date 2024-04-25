mod board; // Import the board module
mod utils;

use board::{Board, draw_board}; // Import Board struct and PIECE_INDEX_MAP

fn main() {
    let board = Board{bitboards: [0;12]};
    draw_board(board);
}

