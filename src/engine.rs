use crate::legalmoves::{generate_legal_moves, unmake_move};
use crate::{algebraic_to_move, board::Board, legalmoves, make_move, utils, BitIter, Move, Turn};
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct ChessEngine {
    board: Board,           // Add fields as needed
    starting_pos_set: bool, // whether the starting position is set to prevent backtracking
    color: Turn,
    rel_value: HashMap<isize, i32>,
}

impl ChessEngine {
    pub fn new() -> Self {
        let board = Board::new(None);
        let starting_pos_set = false;
        let mut rel_value: HashMap<isize, i32> = HashMap::new();
        rel_value.insert(0, 1);
        rel_value.insert(1, 5);
        rel_value.insert(2, 0);
        rel_value.insert(3, 3);
        rel_value.insert(4, 9);
        rel_value.insert(5, 3);
        // Initialize your engine
        ChessEngine {
            board,
            starting_pos_set,
            color: Turn::White,
            rel_value,
        }
    }

    pub fn new_game(&mut self) {
        self.board = Board::new(Some(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        ));
        self.starting_pos_set = false;
    }
    pub fn set_position<'a>(&'a mut self, command: &'a str) {
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
            self.color = self.board.current_state.turn;
        }
        // there is a last move, make that move
        if let Some(m) = lastmove {
            let chess_move = algebraic_to_move(&self.board, m);
            println!("Performed move {}", chess_move);
            make_move(&mut self.board, &chess_move, true);
        }
        self.board.draw();
        self.board.print_state();
    }

    pub fn find_best_move(&mut self, _command: &str) -> String {
        //let best_move = self.find_best_move_minimax(4); // You can adjust the depth as needed
        //
        let best_move = self.find_best_move_alpha_beta(5);
        if let Some(m) = best_move {
            println!("meeko found best move: {}", m);
            make_move(&mut self.board, &m, true);
            self.board.draw();
            self.board.print_state();
            m.alg_move()
        } else {
            "no legal moves".to_string()
        }
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

    /// evaluates the current position on the board

    fn minimax(
        &mut self,
        evaluation: fn(&Board, &HashMap<isize, i32>) -> i32,
        depth: i32,
    ) -> (i32, Option<Move>) {
        if depth == 0 {
            return (evaluation(&self.board, &self.rel_value), None);
        }

        let mut max_value = std::i32::MIN + 1;
        let mut best_move = None;
        let moves = generate_legal_moves(&mut self.board);

        for m in moves {
            make_move(&mut self.board, &m, true);
            let (score, _) = self.minimax(evaluation, depth - 1);
            let score = -score; // Negate the score for the opponent's perspective
            unmake_move(&mut self.board, &m, true); // Undo the move

            if score > max_value {
                max_value = score;
                best_move = Some(m);
            }
        }

        (max_value, best_move)
    }
    pub fn find_best_move_minimax(&mut self, depth: i32) -> Option<Move> {
        let (_, best_move) = self.minimax(relative_value_evaluation, depth);
        best_move
    }

    pub fn alpha_beta(
        &mut self,
        evaluation: fn(&Board, &HashMap<isize, i32>) -> i32,
        depth: i32,
        mut alpha: i32,
        beta: i32,
    ) -> (i32, Option<Move>) {
        if depth == 0 {
            return (evaluation(&self.board, &self.rel_value), None);
        }

        let mut best_move = None;
        let moves = generate_legal_moves(&mut self.board);

        for m in moves {
            make_move(&mut self.board, &m, true);
            let (score, _) = self.alpha_beta(evaluation, depth - 1, -beta, -alpha);
            let score = -score; // Negate the score for the opponent's perspective
            unmake_move(&mut self.board, &m, true); // Undo the move

            if score > alpha {
                alpha = score;
                best_move = Some(m);
            }

            if alpha >= beta {
                break; // Beta cutoff
            }
        }

        (alpha, best_move)
    }

    pub fn find_best_move_alpha_beta(&mut self, depth: i32) -> Option<Move> {
        let (_, best_move) = self.alpha_beta(
            relative_value_evaluation,
            depth,
            std::i32::MIN + 1,
            std::i32::MAX,
        );
        best_move
    }
}
fn relative_value_evaluation(board: &Board, rel_value: &HashMap<isize, i32>) -> i32 {
    let mut white_value = 0;
    for bb_index in 0..6 {
        for bit in BitIter(board.bitboards[bb_index]) {
            white_value += rel_value.get(&(bb_index as isize)).unwrap();
        }
    }
    let mut black_value = 0;
    for bb_index in 6..12 {
        for bit in BitIter(board.bitboards[bb_index]) {
            black_value += rel_value.get(&(bb_index as isize - 6)).unwrap();
        }
    }
    match board.current_state.turn {
        Turn::White => white_value - black_value,
        Turn::Black => black_value - white_value,
    }
}
