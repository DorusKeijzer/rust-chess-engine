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
use legalmoves::rook_attacks;
use legalmoves::unmake_move;
use legalmoves::{make_move, perft};
use utils::{algebraic_to_square, square_to_algebraic};

/// TODO prio order:
/// Debug castling:
///     castling possible when rooks threatened
///     
/// Debug EP:
///     possible need for pin check
///
/// implement check
///     write test
///     make it so only allowed moves are ones that cancel check
/// Debug PERFT
///     1. Write more perft test (GPT ?)
///     2. debug until they all pass
///
/// Minimax with bsimple board state evaluation
/// Quiessence search
/// Improve board state evaluation
/// Zobrist hashing
/// Opening books

fn main() {
    // Get the argument from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <argument 1> <argument 2>", args[0]);
        std::process::exit(1);
    }
    let mode = &args[1];
    let fen = &args[2];
    let mut board = Board::new(Some(fen));
    match mode.as_str() {
        "default" => {
            board.draw();
            board.print_state();
            let square = algebraic_to_square("e5").unwrap();
            let knightmoves = !board.all_white() & legalmoves::KNIGHT_MOVES[square as usize];
        }
        "script" => {
            let p: i32 = perft(&mut board, 1, 1, true);
            println!("{p}");
        }
        "draw" => board.draw(),
        _ => {
            println!("Not a valid mode :^)")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod castling {
        use super::*;
        #[test]
        fn generate_castling_white() {
            let mut board = Board::new(Some("8/8/8/8/8/8/8/R3K2R w KQkq - 0 1"));
            let rook_move = Move {
                from: 63,
                to: 61,
                piece: Piece::Rook, // Rook
                captured: None,
                promotion: None,
                castled: true,
            };
            board.draw();
            let moves = legalmoves::generate_legal_moves(&mut board);
            for m in moves.clone() {
                println!("{m}");
            }
            assert!(
                moves.contains(&rook_move),
                "Kingside castling move not generated"
            );
        }
        #[test]
        fn dont_generate_castling_white() {
            // shouldn't pass because rooks are threatened
            let mut board = Board::new(Some("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1"));
            let rook_move = Move {
                from: 63,
                to: 61,
                piece: Piece::Rook, // Rook
                captured: None,
                promotion: None,
                castled: true,
            };
            board.draw();
            let moves = legalmoves::generate_legal_moves(&mut board);
            for m in moves.clone() {
                println!("{m}");
            }
            assert!(!moves.contains(&rook_move), "Rooks are threatened  ");
        }
        #[test]
        fn dont_generate_castling_blacks() {
            // shouldn't pass because rooks are threatened
            let mut board = Board::new(Some("r3k2r/8/8/8/8/8/8/R3K2R b kq - 0 1"));
            let rook_move = Move {
                from: 0,
                to: 3,
                piece: Piece::Rook, // Rook
                captured: None,
                promotion: None,
                castled: true,
            };
            board.draw();
            let moves = legalmoves::generate_legal_moves(&mut board);
            for m in moves.clone() {
                println!("{m}");
            }
            assert!(!moves.contains(&rook_move), "Rooks are threatened");
        }
        #[test]
        fn generate_castling_black() {
            let mut board = Board::new(Some("r3k2r/8/8/8/8/8/8/8 b kqKQ - 0 1"));
            let rook_move = Move {
                from: 0,
                to: 3,
                piece: Piece::Rook, // Rook
                captured: None,
                promotion: None,
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
                promotion: None,
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
                promotion: None,
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
                promotion: None,
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
                promotion: None,
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
                promotion: None,
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

        fn test_white_pawn_normal_capture_generation() {
            let mut board = Board::new(Some(
                "rnbqkbnr/pppppppp/8/4p3/3P4/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ));
            let pawn_capture = Move {
                from: algebraic_to_square("d4").unwrap(),
                to: algebraic_to_square("e5").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
                castled: false,
            };

            let moves = generate_legal_moves(&mut board);
            assert!(moves.contains(&pawn_capture));
        }
        #[test]
        fn test_white_pawn_normal_capture() {
            let mut board = Board::new(Some("8/8/8/4p3/3P4/8/8/8 w KQkq - 0 1"));
            let pawn_capture = Move {
                from: algebraic_to_square("d4").unwrap(),
                to: algebraic_to_square("e5").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
                castled: false,
            };
            make_move(&mut board, &pawn_capture, true);
            assert_ne!(board.bitboards[0], 0);
            assert_eq!(board.bitboards[6], 0);
        }
        #[test]
        fn test_white_pawn_normal_capture_and_undo() {
            let mut board = Board::new(Some("8/8/8/4p3/3P4/8/8/8 w KQkq - 0 1"));
            let pawn_capture = Move {
                from: algebraic_to_square("d4").unwrap(),
                to: algebraic_to_square("e5").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
                castled: false,
            };
            board.draw();
            make_move(&mut board, &pawn_capture, true);
            board.draw();
            assert_ne!(board.bitboards[0], 0);
            draw_bb(board.bitboards[0]);
            draw_bb(board.bitboards[6]);
            assert_eq!(board.bitboards[6], 0);
            unmake_move(&mut board, &pawn_capture, true);
            board.draw();
            assert_ne!(board.bitboards[0], 0);
            assert_ne!(board.bitboards[6], 0);
        }
        #[test]
        fn test_white_pawn_normal_capture_double() {
            let mut board = Board::new(Some(
                "rnbqkbnr/pppppppp/8/2p1p3/3P4/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ));
            let pawn_capture_1 = Move {
                from: algebraic_to_square("d4").unwrap(),
                to: algebraic_to_square("e5").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
                castled: false,
            };
            let pawn_capture_2 = Move {
                from: algebraic_to_square("d4").unwrap(),
                to: algebraic_to_square("c5").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
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
            let pawn_capture = Move {
                from: algebraic_to_square("e5").unwrap(),
                to: algebraic_to_square("d4").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
                castled: false,
            };

            let moves = generate_legal_moves(&mut board);
            assert!(moves.contains(&pawn_capture));
        }
        fn test_black_pawn_normal_capture_double() {
            let mut board = Board::new(Some(
                "rnbqkbnr/pppppppp/8/4p3/3P1P2/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
            ));
            let pawn_capture_1 = Move {
                from: algebraic_to_square("e5").unwrap(),
                to: algebraic_to_square("d4").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
                castled: false,
            };

            let pawn_capture_2 = Move {
                from: algebraic_to_square("e5").unwrap(),
                to: algebraic_to_square("b4").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
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
        fn test_white_pawn_promotion_queen() {
            let mut board = Board::new(Some("8/1P6/8/8/8/8/8/8 w KQkq - 0 1"));
            let move_str = "b7b8";
            let from = algebraic_to_square(&move_str[0..2]).unwrap();
            let to = algebraic_to_square(&move_str[2..4]).unwrap();
            let promotion_move = Move {
                from,
                to,
                piece: Piece::Pawn,
                captured: None,
                promotion: Some(Piece::Queen),
                castled: false,
            };
            make_move(&mut board, &promotion_move, true);
            // Assert board state after promotion
            assert_eq!(
                board.bitboards[4], // queen must be equal to the square moved to
                utils::mask(to)
            );
            assert_eq!(
                board.bitboards[0], // pawn must be empty
                0
            );
        }
        #[test]
        fn test_white_pawn_promotion_queen_and_unmake() {
            let mut board = Board::new(Some("8/1P6/8/8/8/8/8/8 w KQkq - 0 1"));
            let move_str = "b7b8";
            let from = algebraic_to_square(&move_str[0..2]).unwrap();
            let to = algebraic_to_square(&move_str[2..4]).unwrap();
            let promotion_move = Move {
                from,
                to,
                piece: Piece::Pawn,
                captured: None,
                promotion: Some(Piece::Queen),
                castled: false,
            };
            make_move(&mut board, &promotion_move, true);
            // Assert board state after promotion
            assert_eq!(
                board.bitboards[4], // queen must be equal to the square moved to
                utils::mask(to)
            );
            assert_eq!(
                board.bitboards[0], // pawn must be empty
                0
            );
            unmake_move(&mut board, &promotion_move, true);
            assert_eq!(
                board.bitboards[0], // pawn must be equal to the square moved from after remaking
                utils::mask(from)
            );
            assert_eq!(
                board.bitboards[4], // queen must now be empty
                0
            );
        }
        #[test]
        fn test_black_pawn_promotion_queen() {
            let mut board = Board::new(Some("8/8/8/8/8/8/1p6/8 b KQkq - 0 1"));
            let move_str = "b2b1";
            let from = algebraic_to_square(&move_str[0..2]).unwrap();
            let to = algebraic_to_square(&move_str[2..4]).unwrap();
            let promotion_move = Move {
                from,
                to,
                piece: Piece::Pawn,
                captured: None,
                promotion: Some(Piece::Queen),
                castled: false,
            };
            make_move(&mut board, &promotion_move, true);
            // Assert board state after promotion
            assert_eq!(
                board.bitboards[10], // queen must be equal to the square moved to
                utils::mask(to)
            );
            assert_eq!(
                board.bitboards[6], // pawn must be empty
                0
            );
        }
        #[test]
        fn test_white_pawn_promotion_bishop() {
            let mut board = Board::new(Some("8/1P6/8/8/8/8/8/8 w KQkq - 0 1"));
            let move_str = "b7b8";
            let from = algebraic_to_square(&move_str[0..2]).unwrap();
            let to = algebraic_to_square(&move_str[2..4]).unwrap();
            let promotion_move = Move {
                from,
                to,
                piece: Piece::Pawn,
                captured: None,
                promotion: Some(Piece::Bishop),
                castled: false,
            };
            make_move(&mut board, &promotion_move, true);
            // Assert board state after promotion
            assert_eq!(
                board.bitboards[5], // bishop must be equal to the square moved to
                utils::mask(to)
            );
            assert_eq!(
                board.bitboards[0], // pawn must be empty
                0
            );
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
            let mut board = Board::new(Some("8/8/8/3pP3/8/8/8/8 w KQkq d6 0 1"));
            let move_str = "e5d6";
            let from = algebraic_to_square(&move_str[0..2]).unwrap();
            let to = algebraic_to_square(&move_str[2..4]).unwrap();
            let en_passant_move = Move {
                from,
                to,
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn), // Capturing the pawn en passant
                promotion: None,
                castled: false,
            };
            make_move(&mut board, &en_passant_move, true);
            // Assert board state after en passant capture
            assert_eq!(board.bitboards[0], utils::mask(to)); // white pawn must be on the captured square
            assert_eq!(board.bitboards[6], 0) // black pawn on d5 must be captured
        }

        #[test]
        fn test_white_en_passant_capture_and_unmake() {
            let mut board = Board::new(Some("8/8/8/3pP3/8/8/8/8 w KQkq d6 0 1"));
            let move_str = "e5d6";
            let from = algebraic_to_square(&move_str[0..2]).unwrap();
            let to = algebraic_to_square(&move_str[2..4]).unwrap();
            let en_passant_move = Move {
                from,
                to,
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn), // Capturing the pawn en passant
                promotion: None,
                castled: false,
            };
            make_move(&mut board, &en_passant_move, true);
            unmake_move(&mut board, &en_passant_move, true);

            // Assert board state after unmaking the en passant capture
            assert_eq!(board.bitboards[0], utils::mask(from)); // white pawn must be back to its original square
            assert_eq!(
                board.bitboards[6],
                utils::mask(algebraic_to_square("d5").unwrap())
            ); // black pawn must be back on d5
        }

        #[test]
        fn test_black_en_passant_capture() {
            let mut board = Board::new(Some("8/8/8/8/3Pp3/8/8/8 b KQkq d3 0 1"));
            let move_str = "e4d3";
            let from = algebraic_to_square(&move_str[0..2]).unwrap();
            let to = algebraic_to_square(&move_str[2..4]).unwrap();
            let en_passant_move = Move {
                from,
                to,
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn), // Capturing the pawn en passant
                promotion: None,
                castled: false,
            };
            make_move(&mut board, &en_passant_move, true);

            // Assert board state after en passant capture
            assert_eq!(board.bitboards[6], utils::mask(to)); // black pawn must be on the captured square
            assert_eq!(board.bitboards[0], 0); // white pawn on d4 must be captured
        }

        #[test]
        fn test_black_en_passant_capture_and_unmake() {
            let mut board = Board::new(Some("8/8/8/8/3Pp3/8/8/8 b KQkq d3 0 1"));
            let move_str = "e4d3";
            let from = algebraic_to_square(&move_str[0..2]).unwrap();
            let to = algebraic_to_square(&move_str[2..4]).unwrap();
            let en_passant_move = Move {
                from,
                to,
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn), // Capturing the pawn en passant
                promotion: None,
                castled: false,
            };
            make_move(&mut board, &en_passant_move, true);
            unmake_move(&mut board, &en_passant_move, true);

            // Assert board state after unmaking the en passant capture
            assert_eq!(board.bitboards[6], utils::mask(from)); // black pawn must be back to its original square
            assert_eq!(
                board.bitboards[0],
                utils::mask(algebraic_to_square("d4").unwrap())
            ); // white pawn must be back on d4
        }

        #[test]
        fn test_generate_white_en_passant_move() {
            let mut board = Board::new(Some("8/8/8/3pP3/8/8/8/8 w KQkq d6 0 1"));
            let en_passant_move_str = "e5d6";
            let from = algebraic_to_square(&en_passant_move_str[0..2]).unwrap();
            let to = algebraic_to_square(&en_passant_move_str[2..4]).unwrap();

            let en_passant_move = Move {
                from,
                to,
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
                castled: false,
            };
            let moves = legalmoves::generate_legal_moves(&mut board);
            assert!(
                moves.contains(&en_passant_move),
                "En passant move not generated for white."
            );
        }
        #[test]
        fn test_dont_generate_white_en_passant_move() {
            let mut board = Board::new(Some("8/8/8/r2pP1K/8/8/8/8 w KQkq d6 0 1"));
            let en_passant_move_str = "e5d6";
            let from = algebraic_to_square(&en_passant_move_str[0..2]).unwrap();
            let to = algebraic_to_square(&en_passant_move_str[2..4]).unwrap();

            let en_passant_move = Move {
                from,
                to,
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
                castled: false,
            };
            let moves = legalmoves::generate_legal_moves(&mut board);
            assert!(!moves.contains(&en_passant_move), "King is in check.");
        }

        #[test]
        fn test_generate_black_en_passant_move() {
            let mut board = Board::new(Some("8/8/8/8/3Pp3/8/8/8 b KQkq d3 0 1"));
            let moves = legalmoves::generate_legal_moves(&mut board);

            let en_passant_move_str = "e4d3";
            let from = algebraic_to_square(&en_passant_move_str[0..2]).unwrap();
            let to = algebraic_to_square(&en_passant_move_str[2..4]).unwrap();

            let en_passant_move = Move {
                from,
                to,
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
                castled: false,
            };

            assert!(
                moves.contains(&en_passant_move),
                "En passant move not generated for black."
            );
        }

        // Add more tests for edge cases if needed
    }

    #[cfg(test)]
    mod perft {
        use super::*;
        #[test]
        fn perft_test_1() {
            let mut board: Board =
                Board::new(Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -"));
            let turn: Turn = Turn::White;
            let mut state: State = State {
                turn,
                castling_rights: 0,
                en_passant: None,
            };
            assert_eq!(perft(&mut board, 1, 1, false), 20);
            assert_eq!(perft(&mut board, 2, 2, false), 400);
            assert_eq!(perft(&mut board, 3, 3, false), 8902);
            assert_eq!(perft(&mut board, 4, 4, false), 197_281);
            assert_eq!(perft(&mut board, 5, 5, false), 4_865_609);
        }
        #[test]
        fn perft_test_2() {
            let mut board: Board = Board::new(Some(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
            ));
            let turn: Turn = Turn::White;
            let mut state: State = State {
                turn,
                castling_rights: 0,
                en_passant: None,
            };
            assert_eq!(perft(&mut board, 1, 1, false), 48);
            assert_eq!(perft(&mut board, 2, 2, false), 2039);
            assert_eq!(perft(&mut board, 3, 3, false), 97_862);
            assert_eq!(perft(&mut board, 4, 4, false), 4_085_603);
            assert_eq!(perft(&mut board, 5, 5, false), 193_690_690);
        }
        fn setup_board(fen: &str) -> (Board, State) {
            let board = Board::new(Some(fen));
            let state = State {
                turn: Turn::Black,
                castling_rights: 0,
                en_passant: None,
            };
            (board, state)
        }
    }
}
