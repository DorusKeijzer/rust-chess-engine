mod board; // keeps track of the board
mod legalmoves;
mod utils; // utility functions // legal move generation

use board::Board;
use legalmoves::make_move;
use legalmoves::rook_attacks;
use legalmoves::unmake_move;
use utils::draw_bb;
use utils::BitIter;
fn main() {
    let mut board: Board = Board::new(Some("8/3p4/8/8/8/8/8/8"));
    board.draw();
    let mut state: State = State {
        turn: Turn::Black,
        castling: 0,
        enpassant: 0,
    };
    // let p = legalmoves::perft(&mut board, &mut state, 1,1, true);
    // println!("{}", p);
    let m = legalmoves::generate_legal_moves(&mut board, &mut state);
    for moves in m
    {
        println!("{}", moves)
    }
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

#[derive(PartialEq, Debug)]
pub enum Turn {
    White,
    Black,
}
