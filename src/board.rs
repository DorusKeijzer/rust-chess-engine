use core::panic;
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::utils;

pub const NUMBER_CHARACTERS: usize = 12;
pub const PIECE_CHARACTERS: [char; 12] =
    ['P', 'R', 'K', 'N', 'Q', 'B', 'p', 'r', 'k', 'n', 'q', 'b'];
// returns the index of the pieces
// returns the index of the pieces

lazy_static! {
    /// Maps a piece name to the corresponding bitboard
    static ref PIECE_INDEX_MAP: HashMap<char, usize> = {
        let mut map = HashMap::new();
        map.insert('P', 0);
        map.insert('R', 1);
        map.insert('K', 2);
        map.insert('N', 3);
        map.insert('Q', 4);
        map.insert('B', 5);
        map.insert('p', 6);
        map.insert('r', 7);
        map.insert('k', 8);
        map.insert('n', 9);
        map.insert('q', 10);
        map.insert('b', 11);
        map
    };
}

/// Creates a standard starting board
pub fn standard_start() -> Board {
    let board = Board::new(Some(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    ));
    board
}

/// castling:<br>
/// ```
/// 0 b 0 0 0 0
///     K Q k q
/// ```
#[derive(Clone)]
pub struct State {
    pub turn: Turn,
    pub castling_rights: u8,
    pub enpassant: Option<u8>,
}

impl State {
    pub fn new(state_fen: Option<(&str, &str, &str)>) -> State {
        if let Some(state_string) = state_fen {
            let (turn, castling_string, en_passant) = state_string;
            let whose_turn = match turn.to_lowercase().as_str() {
                "w" => Turn::White,
                "b" => Turn::Black,
                _ => panic!("{turn} is not a valid turn indicator"),
            };
            let castling = 0;
            // Calculate castling rights
            let mut castling = 0;
            if castling_string.contains('K') {
                castling |= 0b1000;
            }
            if castling_string.contains('Q') {
                castling |= 0b0100;
            }
            if castling_string.contains('k') {
                castling |= 0b0010;
            }
            if castling_string.contains('q') {
                castling |= 0b0001;
            }

            let enpassent_square = utils::algebraic_to_square(en_passant);

            State {
                turn: whose_turn,
                castling_rights: castling,
                enpassant: enpassent_square,
            }
        } else {
            State {
                turn: Turn::White,
                castling_rights: 0b1111,
                enpassant: None,
            }
        }
    }
    /// whether the current player can castle kingside
    pub fn can_castle_kingside(&self) -> bool {
        match self.turn {
            Turn::White => (self.castling_rights & 0b0100) != 0,
            Turn::Black => (self.castling_rights & 0b0001) != 0,
        }
    }
    /// whether the current player can castle queenside
    pub fn can_castle_queenside(&self) -> bool {
        match self.turn {
            Turn::White => (self.castling_rights & 0b1000) != 0,
            Turn::Black => (self.castling_rights & 0b0010) != 0,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Turn {
    White,
    Black,
}

/// Stores the state of the board in 12 bit boards. The order is as follows:
///
/// white: P: 0,  R: 1,  K: 2,  N: 3,  Q: 4,  B: 5
///
/// black: p: 6,  r: 7,  k: 8,  n: 9,  q: 10,  b: 11'
pub struct Board {
    pub bitboards: Box<[u64; 12]>,
    pub state_history: Vec<State>,
    pub current_state: State,
}

impl Board {
    pub fn new(fen_string: Option<&str>) -> Self {
        let initial_state = State::new(None);
        let mut board = Self {
            bitboards: Box::new([0; 12]),
            state_history: vec![initial_state.clone()],
            current_state: initial_state,
        };
        
        if let Some(fen) = fen_string {
            let split = fen.split(" ").collect::<Vec<&str>>(); // splits fen string into separate parts
            let new_state = State::new(Some((split[1], split[2], split[3])));
            board.parse_fen(split[0]);
            board.current_state = new_state.clone();
            board.state_history = vec![new_state];
        }
        
        board
    }

    pub fn draw(&self) {
        println!("");
        println!("     A  B  C  D  E  F  G  H");
        println!("");

        let mut result: String = String::from("");
        // this order is used to preserve little-endian indexing
        let mut col = 8;
        for i in (0..64).rev() {
            // when every column in a row is filled, print this row
            if col == 0 {
                col = 8;
                println!("{}  {}", 7 - i / 8, result);
                println!("");
                result = String::from("");
            }
            result.push(' ');
            // iterate over the bitboards in the board
            let mut piecenotfound = true;
            let mut bbindex = 0; // index of the bitboards
            while piecenotfound && bbindex < NUMBER_CHARACTERS {
                let piecebb = self.bitboards[bbindex];

                if utils::bitset(&piecebb, i) {
                    // the character of the piece if a piece is found
                    result.insert(0, PIECE_CHARACTERS[bbindex]);
                    piecenotfound = false;
                }
                bbindex += 1;
            }
            if piecenotfound {
                result.insert(0, '0');
            }

            result.insert(0, ' ');
            result.insert(0, ' ');

            col -= 1;
        }
        println!("8  {}", result);
        println!("");
    }

    #[allow(dead_code)]
    pub fn parse_fen(&mut self, fenstring: &str) {
        // because fen strings do not obey little endian notation,
        // we have to mirror the substrings of each rank
        let split_fen: Vec<&str> = fenstring.split("/").collect();
        let mut reverse_fen = vec![];

        for split in split_fen.iter().rev() {
            let reversed: String = split.chars().rev().collect();
            reverse_fen.push(reversed);
        }

        // If you want to join the reversed parts into a single string separated by spaces, you can do:
        let joined_reverse_fen = reverse_fen.join("/");

        // to iterate over the square indices (backwards, starting at 63)
        let mut index = 64;
        for character in joined_reverse_fen.chars() {
            match character {
                '0'..='8' => index -= character.to_digit(10).unwrap_or(0),
                'P' | 'R' | 'K' | 'N' | 'Q' | 'B' | 'p' | 'r' | 'k' | 'n' | 'q' | 'b' => {
                    // iterates backwards over piece indices
                    self.bitboards[PIECE_INDEX_MAP[&character]] |= 1 << (index - 1);
                    index -= 1;
                }
                _ => {}
            }
        }
    }
    /// Returns a bitboard of all white pieces on the board.
    ///
    /// This function combines the bitboards of white pieces (pawns, knights, bishops, rooks, queens, kings)
    /// into a single bitboard and returns it.
    pub fn all_white(&self) -> u64 {
        let mut res: u64 = 0;
        for bitboard in &self.bitboards[0..6] {
            res |= bitboard; // ORs together all white boards
        }
        res
    }

    /// Returns a bitboard of all black pieces on the board.
    ///
    /// This function combines the bitboards of black pieces (pawns, knights, bishops, rooks, queens, kings)
    /// into a single bitboard and returns it.
    pub fn all_black(&self) -> u64 {
        let mut res: u64 = 0;
        for bitboard in &self.bitboards[6..12] {
            res |= bitboard; // ORs together all black boards
        }
        res
    }

    pub fn occupied(&self) -> u64 {
        return self.all_black() | self.all_white();
    }
}
