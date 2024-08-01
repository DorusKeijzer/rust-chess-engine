use crate::{algebraic_to_move, board::Board, legalmoves, make_move, utils, BitIter, Turn};
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
        rel_value.insert(2, 20);
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
        // Parse the go command, search for the best move, and return it
        //
        let moves = legalmoves::generate_legal_moves(&mut self.board);
        println!("meeko found best move: {}", moves[1]);
        make_move(&mut self.board, &moves[1], true);
        self.board.draw();
        self.board.print_state();
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
    fn evaluate(&self) -> i32 {
        let mut white_value = 0;
        for bb_index in 0..6 {
            for bit in BitIter(self.board.bitboards[bb_index]) {
                white_value += self.rel_value.get(&(bb_index as isize)).unwrap();
            }
        }
        let mut black_value = 0;
        for bb_index in 6..12 {
            for bit in BitIter(self.board.bitboards[bb_index]) {
                black_value += self.rel_value.get(&(bb_index as isize - 6)).unwrap();
            }
        }

        match self.color {
            Turn::White => white_value - black_value,
            Turn::Black => black_value - white_value,
        }
    }
}
