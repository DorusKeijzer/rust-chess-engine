use lazy_static::lazy_static;
use std::collections::HashMap;

pub const NUMBER_CHARACTERS : usize = 12;
pub const PIECE_CHARACTERS: [ char;12 ] = ['P','R','K','N','Q','B','p','r','k','n','q','b'];
// returns the index of the pieces
// returns the index of the pieces

lazy_static! {
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

pub struct Board
{
    pub bitboards : [u64;12],
}

impl Board
{
    #[allow(dead_code)]
    pub fn parse_fen(&mut self, fenstring : &str)
    {
        // to iterate over the square indices (backwards)
        let mut index = 64;  
        for character in fenstring.chars()
        {
            match character {
                '0'..='9' =>{ index -= character.to_digit(10).unwrap_or(0)},
                'P' | 'R' | 'K' | 'N' | 'Q' | 'B' | 'p' | 'r' | 'k' | 'n' | 'q' | 'b' => {
                    // iterates backwards over piece indices
                    self.bitboards[PIECE_INDEX_MAP[&character]] |= 1 << (index - 1);
                    index -= 1;
                    },
                _ => {}
            }
        }
    }
}


pub fn draw_board(board: Board)
{
    println!("");
    println!("     A  B  C  D  E  F  G  H");
    println!("");

    let mut result: String = String::from("");
    // this order is used to preserve little-endian indexing
    let mut j = 0;
    for i in (0..64).rev()
    {
        if j == 8 { 
            j = 0;
            println!("{}   {}", 8- i / 8, result);
            println!("");
            result = String::from("");
        }
        result.push(' ');
        // for every bitboard in the board
        let mut piecenotfound = true;
        let mut bbindex = 0;
        while piecenotfound && bbindex < NUMBER_CHARACTERS
        {
            let piecebb = board.bitboards[bbindex];
            if {
                let mask: u64 = 1 << i;
                mask & piecebb != 0
            } 
            {
                // the character of the piece if a piece is found
                result.push(PIECE_CHARACTERS[bbindex]);
                piecenotfound = false;
            } 
            bbindex += 1;
        }
        if piecenotfound{result.push('0'); }
        
        result.push(' ');
        j += 1;
    }
    println!("{}   {}", 8, result);

}


pub fn draw_bb(bb: u64)
{
    println!("     A  B  C  D  E  F  G  H");
    println!("");

    let mut result: String = String::from("");
    // this order is used to preserve little-endian indexing
    for j in (0..8).rev()
    {
        let k = j * 8;
        let j = 7-j;
        for i in (k..k+8).rev() 
        {
            if {
                let mask: u64 = 1 << i;
                mask & bb != 0
            }
            {result.push_str(" 1 ")}
            else
            {result.push_str(" 0 ");}
        }
        println!("{}   {result}", j+1);
        println!("");
        result = String::from("");

    }
}



