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
/// Stores the state of the board in 12 bit boards. The order is as follows:
///
/// white: P: 0,  R: 1,  K: 2,  N: 3,  Q: 4,  B: 5
///
/// black: p: 6,  r: 7,  k: 8,  n: 9,  q: 10,  b: 11'
pub struct Board {
    pub bitboards: Box<[u64; 12]>,
}

impl Board {
    pub fn new(fen_string: Option<&str>) -> Self {
        let mut board = Self {
            bitboards: Box::new([0; 12]),
        };
        if let Some(fen) = fen_string {
            board.parse_fen(fen);
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
        println!("{}  {}", 8, result);
    }

    #[allow(dead_code)]
    pub fn parse_fen(&mut self, fenstring: &str) {
        println!("{}", fenstring);
        // because fen strings do not obey little endian notation,
        // we have to mirror the substrings of each rank
        let split_fen = fenstring.split("/");
        let mut reverse_fen = vec![];

        for split in split_fen {
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
}
