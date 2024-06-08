mod board; // keeps track of the board
mod legalmoves;
mod utils; // utility functions // legal move generation

use board::{draw_board, Board}; // Import Board struct and PIECE_INDEX_MAP
fn main() {
    let board = Board::new(Some("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1"));
    draw_board(&board);
    let occ = legalmoves::occupied(&board);
    let mut p = legalmoves::get_positive_ray_attacks(occ, legalmoves::Direction::NorthWest, 25);
    p |= legalmoves::get_positive_ray_attacks( occ, legalmoves::Direction::North, 25);
    p |= legalmoves::get_positive_ray_attacks( occ, legalmoves::Direction::NorthEast, 25);
    p |= legalmoves::get_positive_ray_attacks( occ, legalmoves::Direction::East, 25);
    p |= legalmoves::get_negative_ray_attacks( occ, legalmoves::Direction::SouthEast, 25);
    p |= legalmoves::get_negative_ray_attacks( occ, legalmoves::Direction::South, 25);
    p |= legalmoves::get_negative_ray_attacks( occ, legalmoves::Direction::SouthWest, 25);
    p |= legalmoves::get_negative_ray_attacks( occ, legalmoves::Direction::West, 25);
    utils::draw_bb(p);
    legalmoves::generate_legal_moves(&board, &Turn::White);
}
#[allow(dead_code)]
struct Move {
    from: u8,
    to: u8,
    capture: bool,
}

pub enum Turn {
    White,
    Black,
}

fn make_move(board: &mut Board, chess_move: Move) {
    let from = chess_move.from;
    let to = chess_move.to;
    if let Some(index) = utils::find_bitboard(board, from) {
        utils::move_piece(&mut board.bitboards[index], to, from);
    }
}
