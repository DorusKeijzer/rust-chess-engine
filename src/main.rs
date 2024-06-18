#![allow(dead_code, unused_parens, unused_variables, unused_imports)]

mod board; // keeps track of the board
mod legalmoves;
mod utils; // utility functions // legal move generation
use std::env;

use std::fs::Permissions;

use crate::{
    board::{Board, State, Turn},
    legalmoves::{Move, Piece},
    utils::{draw_bb, find_bitboard, BitIter},
};
use legalmoves::{make_move, perft};
use legalmoves::rook_attacks;
use legalmoves::unmake_move;

/// TODO prio order:
/// Investigate if there is a problem with pawn captures
///     1. Generate tests for pawn captures
///     2. if tests fail, fix pawn captures
/// Implement promotion
///     1. Generate tests for promotions
///     2. update struct and functions to accomodate promotions
///     3. etc.
/// Implement en passant
///     1. Generate tests for en passant
///     2. update struct and functions to accomodate en passant
///     3. etc.
/// Debug PERFT
///     1. Write more perft test (GPT ?)
///     2. debug until they all pass
/// Simple board state evaluation
/// Minimax
/// Quiessence search
/// Improve board state evaluation
/// Zobrist hashing
/// Opening books

fn main() {
    // Get the argument from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <argument>", args[0]);
        std::process::exit(1);
    }
    let fen = &args[1];
    
    let mut board = Board::new(Some(fen));
    let p = perft(&mut board, 1, 1, true);
    println!("{p}");
}

#[cfg(test)]
mod tests {
    use super::*;
    mod castling {
        use super::*;
        #[test]
        fn castling_in_make_moves_for_white() {
            let mut board = Board::new(Some("r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1"));
            let rook_move = Move {
                from: 63,
                to: 61,
                piece: Piece::Rook, // Rook
                captured: None,
                castled: true,
            };
            let moves = legalmoves::generate_legal_moves(&mut board);

            assert!(moves.contains(&rook_move));
        }
        #[test]
        fn castling_in_make_moves_for_black() {
            let mut board = Board::new(Some("r3k2r/8/8/8/8/8/8/R3K2R b kqKQ - 0 1"));
            let rook_move = Move {
                from: 0,
                to: 3,
                piece: Piece::Rook, // Rook
                captured: None,
                castled: true,
            };
            let moves = legalmoves::generate_legal_moves(&mut board);

            assert!(moves.contains(&rook_move));
        }
        #[test]
        fn test_non_castling_move_loses_castling_rights_king() {
            let mut board = Board::new(Some("r3k2r/8/8/8/8/8/8/R3K2R b KQ - 0 1"));

            // Move the white king
            let king_move = Move {
                from: 4,
                to: 12,
                piece: Piece::King, // King
                captured: None,
                castled: false,
            };

            legalmoves::make_move(&mut board, &king_move, true);

            assert_eq!(board.current_state.castling_rights & 0b0011, 0b0000); // White lost both castling rights
        }

        #[test]
        fn test_non_castling_move_loses_castling_rights_rook() {
            let mut board: Board = Board::new(Some("r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1"));
            assert_eq!(board.current_state.castling_rights, 0b1100);
            // Move the white kingside rook
            let rook_move = Move {
                from: 63,
                to: 55,
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
            let mut board: Board = Board::new(Some("r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1"));
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
            let mut board: Board = Board::new(Some("r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1"));

            // Perform kingside castling for White
            let rook_move = Move {
                from: 63,
                to: 61,
                piece: Piece::Rook, // Rook
                captured: None,
                castled: true,
            };

            let (king_move, _) = legalmoves::reconstruct_king_move(&rook_move, &board);

            make_move(&mut board, &rook_move, true);
            assert_eq!(board.current_state.castling_rights, 0b0000); // Castling rights should be restored
                                                                     // Unmake the moves
            unmake_move(&mut board, &rook_move, true);
            assert_eq!(board.current_state.castling_rights, 0b1100); // Castling rights should be restored
            assert_eq!(board.current_state.turn, Turn::White); // It should still be White's turn
        }

        #[test]
        fn test_double_check_castling_rights() {
            let mut board: Board = Board::new(Some("r3k2r/8/8/8/8/8/8/R3K2R w KQ - 0 1"));

            // Perform queenside castling for White
            let rook_move = Move {
                from: 56,
                to: 59,
                piece: Piece::Rook, // Rook
                captured: None,
                castled: true,
            };

            let (king_move, _) = legalmoves::reconstruct_king_move(&rook_move, &board);

            make_move(&mut board, &rook_move, true);

            assert_eq!(board.current_state.castling_rights & 0b1100, 0b0000); // White lost all castling rights

            // Unmake the moves
            unmake_move(&mut board, &rook_move, true);

            assert_eq!(board.current_state.castling_rights, 0b1100); // Castling rights should be restored
            assert_eq!(board.current_state.turn, Turn::White); // It should still be White's turn
        }
    }

    #[cfg(test)]
    mod pawn_captures_tests {
        use legalmoves::generate_legal_moves;

        use super::*;
        use crate::{
            board::Board,
            legalmoves::{Move, Piece},
            utils::algebraic_to_square,
        };

        #[test]
        
        fn test_white_pawn_normal_capture() {
            let mut board = Board::new(Some(
                "rnbqkbnr/pppppppp/8/4p3/3P4/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ));
            let pawn_capture = Move
            {
                from: algebraic_to_square( "d4").unwrap(),
                to: algebraic_to_square( "e5").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                castled: false, 
            };

            let moves = generate_legal_moves(&mut board);
            assert!(moves.contains(&pawn_capture));
        }
        #[test]
        fn test_white_pawn_normal_capture_double() {
            let mut board = Board::new(Some(
                "rnbqkbnr/pppppppp/8/2p1p3/3P4/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ));
            let pawn_capture_1 = Move
            {
                from: algebraic_to_square( "d4").unwrap(),
                to: algebraic_to_square( "e5").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                castled: false, 
            };
            let pawn_capture_2 = Move
            {
                from: algebraic_to_square( "d4").unwrap(),
                to: algebraic_to_square( "c5").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                castled: false, 
            };
            board.draw();
            let moves = generate_legal_moves(&mut board);
            assert!(moves.contains(&pawn_capture_1));
            assert!(moves.contains(&pawn_capture_2));
        }
        #[test]
        fn test_black_pawn_normal_capture() {
            let mut board = Board::new(Some(
                "rnbqkbnr/pppppppp/8/4p3/3P4/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
            ));
            let pawn_capture = Move
            {
                from: algebraic_to_square( "e5").unwrap(),
                to: algebraic_to_square( "d4").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                castled: false, 
            };

            let moves = generate_legal_moves(&mut board);
            assert!(moves.contains(&pawn_capture));
        }
        fn test_black_pawn_normal_capture_double() {
            let mut board = Board::new(Some(
                "rnbqkbnr/pppppppp/8/4p3/3P1P2/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
            ));
            let pawn_capture_1 = Move
            {
                from: algebraic_to_square( "e5").unwrap(),
                to: algebraic_to_square( "d4").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                castled: false, 
            };

            let pawn_capture_2 = Move
            {
                from: algebraic_to_square( "e5").unwrap(),
                to: algebraic_to_square( "b4").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                castled: false, 
            };

            let moves = generate_legal_moves(&mut board);
            assert!(moves.contains(&pawn_capture_1));
            assert!(moves.contains(&pawn_capture_2));
        }
        // Add more tests for black pawn captures if needed
    }

    #[cfg(test)]
    mod pawn_promotion_tests {
        use super::*;
        use crate::{
            board::Board,
            legalmoves::{Move, Piece},
            utils::algebraic_to_square,
        };

        #[test]
        fn test_white_pawn_promotion() {
            let mut board = Board::new(Some("rnbqkbnr/1P6/8/8/8/8/8/RNBQKBNR w KQkq - 0 1"));
            let move_str = "b7b8";
            let from = algebraic_to_square(&move_str[0..2]).unwrap();
            let to = algebraic_to_square(&move_str[2..4]).unwrap();
            let promotion_move = Move {
                from,
                to,
                piece: Piece::Pawn,
                captured: None,
                castled: false,
            };
            make_move(&mut board, &promotion_move, true);

            // Assert board state after promotion
            assert_eq!(
                board.bitboards[Piece::Queen as usize],
                0x0000_0000_0000_0002
            ); // Adjust this according to your board representation
        }

        // Add more tests for black pawn promotions if needed
    }

    #[cfg(test)]
    mod en_passant_tests {
        use super::*;
        use crate::{
            board::Board,
            legalmoves::{Move, Piece},
            utils::algebraic_to_square,
        };

        #[test]
        fn test_white_en_passant_capture() {
            let mut board = Board::new(Some(
                "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 1",
            ));
            let move_str = "f5e6";
            let from = algebraic_to_square(&move_str[0..2]).unwrap();
            let to = algebraic_to_square(&move_str[2..4]).unwrap();
            let en_passant_move = Move {
                from,
                to,
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn), // Capturing the pawn en passant
                castled: false,
            };
            make_move(&mut board, &en_passant_move, true);

            // Assert board state after en passant capture
            assert_eq!(board.bitboards[Piece::Pawn as usize], 0x0000_0000_0000_00D0);
            // Adjust this according to your board representation
        }

        // Add more tests for black en passant captures if needed
    }
}
