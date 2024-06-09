mod board; // keeps track of the board
mod legalmoves;
mod utils; // utility functions // legal move generation

use board::Board;
use legalmoves::make_move;
use legalmoves::rook_attacks;
use legalmoves::unmake_move;
use utils::draw_bb;
fn main() {
    let mut board: Board = Board::new(Some("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR"));
    board.draw_board();
    let state: State = State {
        turn: Turn::White,
        castling: 0,
        enpassant: 0,
    };

    let p = legalmoves::perft(&mut board, &state, 1, true);
    println!("{:?}", p);
}

pub struct State {
    turn: Turn,
    castling: u8,
    enpassant: u8,
}

impl State {
    pub fn new() -> State {
        State {
            turn: Turn::White,
            castling: 0b1111,
            enpassant: 0,
        }
    }
}

#[derive(PartialEq)]
pub enum Turn {
    White,
    Black,
}
