mod board; // keeps track of the board
mod legalmoves;
mod utils; // utility functions // legal move generation
use std::env;

use crate::{
    board::{Board, State, Turn},
    legalmoves::{Move, Piece},
    utils::{draw_bb, find_bitboard, BitIter},
};
use legalmoves::{format_for_debug, make_move, perft};
use legalmoves::{generate_legal_moves, rook_attacks};
use std::collections::VecDeque;
use utils::{algebraic_to_square, square_to_algebraic};

/// TODO prio order:
/// Debug EP:
///     
///     EP is not properly transfered between moves
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
use std::io::{self, BufRead, Write};

fn main() {
    let mut engine = ChessEngine::new(); // You'll need to implement this

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        stdout.flush().unwrap();
        stdin.lock().read_line(&mut buffer).unwrap();

        let input = buffer.trim();

        match input {
            "uci" => {
                println!("id name Meeko");
                println!("id author YourName");
                // Add any options here
                println!("uciok");
            }
            "isready" => println!("readyok"),
            "ucinewgame" => engine.new_game(),
            "quit" => break,
            _ if input.starts_with("position") => engine.set_position(input),
            _ if input.starts_with("go") => {
                let best_move = engine.find_best_move(input);
                println!("bestmove {}", best_move);
            }
            _ => println!("Unknown command: {}", input),
        }
    }
}

struct ChessEngine {
    board: Board,           // Add fields as needed
    starting_pos_set: bool, // whether the starting position is set to prevent backtracking
}

impl ChessEngine {
    fn new() -> Self {
        let board = Board::new(None);
        let starting_pos_set = false;
        // Initialize your engine
        ChessEngine {
            board,
            starting_pos_set,
        }
    }

    fn new_game(&mut self) {
        self.board = Board::new(Some(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        ));
        self.starting_pos_set = false;
    }
    fn set_position<'a>(&'a mut self, command: &'a str) {
        // Parse the position command and set up the board
        let (fen, lastmove) = self.parse_position(command);

        // ignore fen if already set
        if !self.starting_pos_set {
            self.board = match fen.as_str() {
                "startpos" => Board::new(Some(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                )),
                _ => Board::new(Some(&fen)),
            };
            self.starting_pos_set = true;
        }
        // there is a last move, make that move
        if let Some(m) = lastmove {
            let chess_move = algebraic_to_move(&self.board, m);
            make_move(&mut self.board, &chess_move, true);
        }
        self.board.draw();
    }

    fn find_best_move(&mut self, _command: &str) -> String {
        // Parse the go command, search for the best move, and return it
        //
        let moves = legalmoves::generate_legal_moves(&mut self.board);

        make_move(&mut self.board, &moves[1], true);
        moves[1].alg_move()
    }

    /// parses a string such as "position fen bla bla bla moves a1a2"
    /// returns the fen string and the last performed move
    fn parse_position<'a>(&mut self, command: &'a str) -> (String, Option<&'a str>) {
        // splits command at every whitespace and turns into a double queue
        let words: Vec<&str> = command.split_whitespace().collect();
        let mut deque: VecDeque<&str> = VecDeque::from(words);
        // logic for separating fen from moves
        let mut registerfen = false;
        let mut fen = String::new();
        while let Some(current) = deque.pop_front() {
            println!("{}", current);
            if registerfen {
                fen.push_str(current);
                fen.push(' ');
            }
            match current {
                // after word fen, start registering every command as part of fen string
                // until encountering "moves"
                "startpos" => {
                    fen = current.to_string();
                    break;
                }
                "fen" => {
                    registerfen = true;
                }
                "moves" => {
                    break;
                }
                _ => {}
            };
        }
        let lastmove = deque.pop_back();
        (fen, lastmove)
    }
}

fn algebraic_to_move(board: &Board, algebraic_string: &str) -> Move {
    let from = algebraic_to_square(&algebraic_string[0..2]).unwrap();
    let to = algebraic_to_square(&algebraic_string[2..4]).unwrap();

    let promotion = match &algebraic_string.chars().nth(4) {
        Some('q') => Some(Piece::Queen),
        Some('r') => Some(Piece::Rook),
        Some('b') => Some(Piece::Bishop),
        Some('n') => Some(Piece::Knight),
        Some(_) => None,
        None => None,
    };
    let piece = match find_bitboard(&board, from).map(|x| x % 6) {
        Some(0) => Piece::Pawn,
        Some(1) => Piece::Rook,
        Some(2) => Piece::King,
        Some(3) => Piece::Knight,
        Some(4) => Piece::Queen,
        Some(5) => Piece::Bishop,
        _ => panic!(),
    };
    let captured = match find_bitboard(&board, to).map(|x| x % 6) {
        Some(0) => Some(Piece::Pawn),
        Some(1) => Some(Piece::Rook),
        Some(2) => Some(Piece::King),
        Some(3) => Some(Piece::Knight),
        Some(4) => Some(Piece::Queen),
        Some(5) => Some(Piece::Bishop),
        _ => None,
    };

    let castled = piece == Piece::King && (from == 4 && (to == 6 || to == 2))
        || (from == 60 && (to == 62 || to == 58));

    Move {
        from,
        to,
        piece,
        promotion,
        captured,
        castled: false,
    }
}

fn old_main() {
    // Get the argument from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {}
    if args.len() != 3 && args.len() != 4 {
        println!("Usage: {} <argument 1> <argument 2>", args[0]);
        std::process::exit(1);
    }
    let mode = &args[1];
    let fen = &args[2];

    let mut board = Board::new(Some(fen));
    match mode.as_str() {
        "default" => {
            let mut board = Board::new(Some(
                "r3k2r/p1ppqpb1/Bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPB1PPP/R3K2R w KQkq - ",
            ));
            board.draw();
            let legalmoves = generate_legal_moves(&mut board);
            utils::draw_bb(legalmoves::attacks(&board, Turn::Black))
        }
        "script" => {
            let p: i32 = perft(&mut board, 1, 1, true);
            println!("{p}");
        }
        "quiet" => {
            let depth: i32 = args[3].parse().unwrap();
            let p: i32 = perft(&mut board, depth, depth, false);
            println!("{p}");
        }
        "draw" => board.draw(),
        "debug" => {
            let depth: i32 = args[3].parse().unwrap();
            assert!(depth > 1, "Depth must be greater than 1");
            format_for_debug(board, depth);
        }
        _ => {
            println!("Not a valid mode :^)")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod generation {
        use super::*;
        #[test]
        fn start_from_check() {
            let mut board: Board = Board::new(Some(
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            ));
            board.draw();
            board.print_state();

            println!("moves:");

            let legalmoves = generate_legal_moves(&mut board);
            for m in legalmoves.clone() {
                println!("{}", m);
            }

            assert_eq!(legalmoves.len(), 6, "6 possible moves to get out of check");
        }
    }
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
        fn rooks_threatened_white() {
            // it should be irrelevant whether rooks are threatened
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
            assert!(
                moves.contains(&rook_move),
                "Doesn't matter if rooks are threatened  "
            );
        }
        #[test]
        fn king_in_check() {
            // it should be irrelevant whether rooks are threatened
            let mut board = Board::new(Some("r3k2r/8/8/4r3/8/8/8/R3K2R w KQkq - 0 1"));
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
                !moves.contains(&rook_move),
                "Doesn't matter if rooks are threatened  "
            );
        }
        #[test]
        fn rooks_threatened_back() {
            // it should be irrelevant if rooks are threatened
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
            assert!(
                moves.contains(&rook_move),
                "Doesn't matter if rooks are threatened"
            );
        }
        #[test]
        fn rooks_path_threatened() {
            // it should be irrelevant if rooks are threatened
            let mut board = Board::new(Some("r3k2r/8/8/8/8/8/8/1R2K2R b kq - 0 1"));
            let rook_move = Move {
                from: 0,
                to: 3,
                piece: Piece::Rook, // Rook
                captured: None,
                promotion: None,
                castled: true,
            };
            println!("AAAAAAAAA");
            board.draw();
            let moves = legalmoves::generate_legal_moves(&mut board);
            for m in moves.clone() {
                println!("{m}");
            }
            assert!(
                moves.contains(&rook_move),
                "rook path being threatened is not relevant"
            );
        }
        #[test]
        fn kings_path_threatened() {
            // it should be irrelevant if rooks are threatened
            let mut board = Board::new(Some("r3k2r/8/8/8/8/8/8/3RKR2 b kq - 0 1"));
            let rook_move = Move {
                from: 0,
                to: 3,
                piece: Piece::Rook, // Rook
                captured: None,
                promotion: None,
                castled: true,
            };

            println!("BBBBBBBBB");
            board.draw();
            let moves = legalmoves::generate_legal_moves(&mut board);
            for m in moves.clone() {
                println!("{m}");
            }
            assert!(!moves.contains(&rook_move), "kings path is threatened");
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

        #[test]
        fn test_white_en_passant_capture_on_a_file() {
            let mut board = Board::new(Some("8/8/8/pP6/8/8/8/8 w - a6 0 1"));
            let moves = legalmoves::generate_legal_moves(&mut board);
            let en_passant_move = Move {
                from: algebraic_to_square("b5").unwrap(),
                to: algebraic_to_square("a6").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
                castled: false,
            };
            assert!(
                moves.contains(&en_passant_move),
                "En passant move not generated on a-file for white."
            );
        }

        #[test]
        fn test_black_en_passant_capture_on_h_file() {
            let mut board = Board::new(Some("8/8/8/8/5Pp1/8/8/8 b - f3 0 1"));
            let moves = legalmoves::generate_legal_moves(&mut board);
            let en_passant_move = Move {
                from: algebraic_to_square("g4").unwrap(),
                to: algebraic_to_square("f3").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
                castled: false,
            };
            assert!(
                moves.contains(&en_passant_move),
                "En passant move not generated on h-file for black."
            );
        }

        #[test]
        fn test_en_passant_not_available_after_one_move() {
            let mut board = Board::new(Some("8/8/8/3pP3/8/8/8/8 w - d6 0 1"));
            let moves = legalmoves::generate_legal_moves(&mut board);
            let non_ep_move = Move {
                from: algebraic_to_square("e5").unwrap(),
                to: algebraic_to_square("e6").unwrap(),
                piece: Piece::Pawn,
                captured: None,
                promotion: None,
                castled: false,
            };
            make_move(&mut board, &non_ep_move, true);

            let black_moves = legalmoves::generate_legal_moves(&mut board);
            let invalid_ep_move = Move {
                from: algebraic_to_square("d5").unwrap(),
                to: algebraic_to_square("e6").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
                castled: false,
            };
            assert!(
                !black_moves.contains(&invalid_ep_move),
                "En passant move should not be available after one move."
            );
        }

        #[test]
        fn test_en_passant_capture_updates_board_state() {
            let mut board = Board::new(Some("8/8/8/3pP3/8/8/8/8 w - d6 0 1"));
            let ep_move = Move {
                from: algebraic_to_square("e5").unwrap(),
                to: algebraic_to_square("d6").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
                castled: false,
            };
            board.draw();
            make_move(&mut board, &ep_move, true);
            board.draw();
            assert_eq!(
                board.bitboards[0],
                utils::mask(algebraic_to_square("d6").unwrap()),
                "White pawn should be on d6"
            );
            assert_eq!(
                board.bitboards[6] & utils::mask(algebraic_to_square("d5").unwrap()),
                0,
                "Black pawn should be removed from d5"
            );

            assert_eq!(
                board.current_state.en_passant, None,
                "En passant square should be reset after capture"
            );
        }
        #[test]
        fn test_en_passant_state_generation() {
            let mut board = Board::new(Some(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
            ));
            board.draw();

            let pawn_move = legalmoves::Move {
                from: algebraic_to_square("g2").unwrap(),

                to: algebraic_to_square("g4").unwrap(),
                piece: Piece::Pawn,
                captured: None,
                promotion: None,
                castled: false,
            };

            make_move(&mut board, &pawn_move, true);
            board.draw();
            board.print_state();

            assert_ne!(
                board.current_state.en_passant, None,
                "G2 should be the ep square"
            );
        }

        #[test]
        fn test_en_passant_capture_in_check() {
            let mut board = Board::new(Some("8/8/8/3pP3/7k/7R/8/7K w - d6 0 1"));
            let moves = legalmoves::generate_legal_moves(&mut board);
            let ep_move = Move {
                from: algebraic_to_square("e5").unwrap(),
                to: algebraic_to_square("d6").unwrap(),
                piece: Piece::Pawn,
                captured: Some(Piece::Pawn),
                promotion: None,
                castled: false,
            };

            for m in moves.clone() {
                println!("{}", m);
            }
            assert!(
                moves.contains(&ep_move),
                "En passant capture should be allowed when not in check."
            );

            board.draw();
            // Now put the king in check
            let mut board = Board::new(Some("8/8/8/3pP3/7k/8/8/r6K w - d6 0 1"));
            let moves = legalmoves::generate_legal_moves(&mut board);
            board.draw();

            for m in moves.clone() {
                println!("{}", m);
            }
            assert!(
                !moves.contains(&ep_move),
                "En passant capture should not be allowed when in check."
            );
        }
    }
    #[cfg(test)]
    mod perft {
        use super::*;

        fn run_perft_test(fen: &str, expected_results: &[(u32, u64)]) {
            let mut board = Board::new(Some(fen));
            for (depth, expected) in expected_results {
                let result = perft(&mut board, *depth as i32, *depth as i32, false);
                if result != *expected as i32 {
                    println!("Test failed at depth {}", depth);
                    println!("FEN: {}", fen);
                    println!("Expected: {}, Got: {}", expected, result);
                    panic!("Perft test failed");
                }
            }
        }

        #[test]
        fn perft_test_1() {
            let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";
            let expected = vec![(1, 20), (2, 400), (3, 8902), (4, 197_281), (5, 4_865_609)];
            run_perft_test(fen, &expected);
        }

        #[test]
        fn perft_test_2() {
            let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
            let expected = vec![
                (1, 48),
                (2, 2039),
                (3, 97_862),
                (4, 4_085_603),
                (5, 193_690_690),
            ];
            run_perft_test(fen, &expected);
        }

        #[test]
        fn perft_test_3() {
            let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -  ";
            let expected = vec![(1, 14), (2, 191), (3, 2812), (4, 43_238), (5, 746_624)];
            run_perft_test(fen, &expected);
        }

        #[test]
        fn perft_test_4() {
            let fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
            let expected = vec![(1, 6), (2, 264), (3, 9467), (4, 422_333), (5, 15_833_292)];
            run_perft_test(fen, &expected);
        }

        #[test]
        fn perft_test_5() {
            let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ";
            let expected = vec![
                (1, 44),
                (2, 1486),
                (3, 62_379),
                (4, 2_103_487),
                (5, 89_941_194),
            ];
            run_perft_test(fen, &expected);
        }

        #[test]
        fn perft_test_6() {
            let fen = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
            let expected = vec![
                (1, 46),
                (2, 2097),
                (3, 89_890),
                (4, 3_894_594),
                (5, 164_076_551),
            ];
            run_perft_test(fen, &expected);
        }
    }
}
