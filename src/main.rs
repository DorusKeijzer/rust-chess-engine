#![allow(dead_code, unused_parens, unused_variables, unused_imports)]

mod board; // keeps track of the board
mod legalmoves;
mod utils; // utility functions // legal move generation

use std::fs::Permissions;

use crate::{
    board::{Board, State, Turn},
    utils::{draw_bb, find_bitboard, BitIter},
    legalmoves::{Move, Piece},
};
use legalmoves::make_move;
use legalmoves::rook_attacks;
use legalmoves::unmake_move;

/// TODO:   1. if non castling move, lose castling rights
///             a. king
///             b. rook
///     	2. double check castling rights
///         3. if castling move is made, reconstruct and make king move
///             a. make_move
///             b. unmake_move
fn main() {
    let mut board = Board::new(Some("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1"));
    board.draw();
    let rook_move = Move {
        from: 63,
        to: 61,
        piece: Piece::Rook, // Rook's piece representation
        captured: None,
        castled: true,
    };

    let (king_move,  _) = legalmoves::reconstruct_king_move(&rook_move, &board);
    println!("King's Move: {}", king_move);
    println!("Performing move {rook_move}");
    make_move(&mut board, &rook_move, false);
    println!("Performing move {king_move}");
    make_move(&mut board, &king_move, true);
    board.draw();
    }



    #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_castling_move_loses_castling_rights_king() {
        let mut board = Board::new(Some("r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1"));
        
        // Move the white king
        let king_move = Move {
            from: 4,
            to: 12,
            piece: Piece::King, // King
            captured: None,
            castled: false,
        };
        
        legalmoves::make_move(&mut board, &king_move, true);
        
        assert_eq!(board.current_state.castling_rights & 0b1100, 0b0000); // White lost both castling rights
    }
    
    #[test]
    fn test_non_castling_move_loses_castling_rights_rook() {
        let mut board: Board= Board::new(Some("r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1"));
        
        // Move the white kingside rook
        let rook_move = Move {
            from: 7,
            to: 15,
            piece: Piece::Rook, // Rook
            captured: None,
            castled: false,
        };
        
        make_move(&mut board, &rook_move, true);
        
        assert_eq!(board.current_state.castling_rights & 0b1000, 0b0000); // White lost kingside castling right
        assert_eq!(board.current_state.castling_rights & 0b0100, 0b0100); // White still has queenside castling right
    }
    
    #[test]
    fn test_castling_rights_after_castling_move() {
        let mut board: Board= Board::new(Some("r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1"));
        // Perform kingside castling for White
        let rook_move = Move {
            from: 63,
            to: 61,
            piece: Piece::Rook, // Rook
            captured: None,
            castled: true,
        };
                
        make_move(&mut board, &rook_move, true);
        
        assert_eq!(board.current_state.castling_rights & 0b1100, 0b0000); // White lost all castling rights
    }
    
    #[test]
    fn test_make_and_unmake_castling_move() {
        let mut board: Board= Board::new(Some("r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1"));
        
        // Perform kingside castling for White
        let rook_move = Move {
            from: 7,
            to: 5,
            piece: Piece::Rook, // Rook
            captured: None,
            castled: true,
        };
        
        let (king_move, _) = legalmoves::reconstruct_king_move(&rook_move, &board);
        
        make_move(&mut board, &rook_move, true);
        
        // Unmake the moves
        unmake_move(&mut board, &king_move, true);
        
        assert_eq!(board.current_state.castling_rights, 0b1100); // Castling rights should be restored
        assert_eq!(board.current_state.turn, Turn::White); // It should still be White's turn
    }
    
    #[test]
    fn test_double_check_castling_rights() {
        let mut board: Board= Board::new(Some("r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1"));
        
        // Perform queenside castling for White
        let rook_move = Move {
            from: 0,
            to: 3,
            piece: Piece::Rook, // Rook
            captured: None,
            castled: true,
        };
        
        let (king_move, _) = legalmoves::reconstruct_king_move(&rook_move, &board);
        
        make_move(&mut board, &rook_move, true);
        
        assert_eq!(board.current_state.castling_rights & 0b1100, 0b0000); // White lost all castling rights
        
        // Unmake the moves
        unmake_move(&mut board, &king_move, true);
        
        assert_eq!(board.current_state.castling_rights, 0b1100); // Castling rights should be restored
        assert_eq!(board.current_state.turn, Turn::White); // It should still be White's turn
    }
}
