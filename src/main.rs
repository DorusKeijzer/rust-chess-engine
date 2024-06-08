mod board; // keeps track of the board
mod legalmoves;
mod utils; // utility functions // legal move generation

use board::{Board};
use legalmoves::make_move; // Import Board struct and PIECE_INDEX_MAP
fn main() {
    let mut board: Board = Board::new(Some("N7/8/8/8/8/8/8/8"));
    let turn: Turn = Turn::White;
    let num = legalmoves::perft(&mut board, &turn,2);
    print!("{:?}", num)
}


#[derive(PartialEq)]
pub enum Turn {
    White,
    Black,
}

