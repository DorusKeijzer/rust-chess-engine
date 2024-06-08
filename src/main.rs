mod board; // keeps track of the board
mod legalmoves;
mod utils; // utility functions // legal move generation

use board::Board;
use legalmoves::make_move;
use legalmoves::unmake_move;
fn main() {
    let mut board: Board = Board::new(Some("Rn6/8/8/8/8/8/8/8"));
    let chess_move = legalmoves::Move {
        from: 56,
        to: 57,
        piece: legalmoves::Piece::Rook,
        captured: Some(legalmoves::Piece::Knight),
    };
    println!("Before");
    board.draw_board();
    make_move(&mut board, &chess_move, &Turn::White);
    println!("Make move");

    board.draw_board();
    unmake_move(&mut board, &chess_move, &Turn::White);
    println!("Undo");

    board.draw_board();
    
}

#[derive(PartialEq)]
pub enum Turn {
    White,
    Black,
}
