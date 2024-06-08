mod board; // keeps track of the board
mod legalmoves;
mod utils; // utility functions // legal move generation

use board::Board;
use legalmoves::make_move;
use legalmoves::rook_attacks;
use legalmoves::unmake_move;
use utils::draw_bb;
fn main() {
    let mut board: Board = Board::new(Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
    board.draw_board();
    let own = legalmoves::all_white(&board);
    utils::draw_bb(legalmoves::rook_attacks(legalmoves::occupied(&board), own, 3));
    utils::draw_bb(legalmoves::bishop_attacks(legalmoves::occupied(&board), own, 3));
    let p = legalmoves::perft(&mut board, &Turn::White, 2);
    println!("{:?}", p);

}

pub struct state
{
    turn: Turn,
    castling: u8,
    enpassant: u8,
}

#[derive(PartialEq)]
pub enum Turn {
    White,
    Black,
}
