mod board; // keeps track of the board
mod utils; // utility functions
mod legalmoves; // legal move generation

use board::{Board, draw_board}; // Import Board struct and PIECE_INDEX_MAP

fn main() {
    let board = Board{bitboards: [0;12]};
    draw_board(board);
}

struct Move
{
    from: u8,
    to: u8,
    capture: bool,
}

fn make_move(board: Board, chess_move: Move) -> ()
{
    
}