mod board; // keeps track of the board
mod utils; // utility functions
mod legalmoves; // legal move generation

use board::{Board, draw_board}; // Import Board struct and PIECE_INDEX_MAP

fn main() {
    let board = Board::new();
    let mut i = 0;
    while i < 64
    {
        println!("{}",i);
        utils::draw_bb(utils::mask(i));
        utils::draw_bb(legalmoves::KNIGHT_MOVES[i as usize]);
        i+=1;
    }
}
#[allow(dead_code)]
struct Move
{
    from: u8,
    to: u8,
    capture: bool,
}

pub enum Turn
{
    White,
    Black
}

fn make_move(board: &mut Board, chess_move: Move) {
    let from = chess_move.from;
    let to = chess_move.to;
    if let Some(index) = utils::find_bitboard(board, from) {
        utils::move_piece(&mut board.bitboards[index], to, from);
    }
}
