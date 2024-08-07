use std::fmt::Error;

use crate::board::Board;
#[allow(dead_code)]
/// Creates a mask from a given index  
pub fn mask(index: u8) -> u64 {
    1 << index
}
#[allow(dead_code)]
/// Checks if a specific bit is set in a given bitboard.
///
/// # Arguments
///
/// * `bb` - The bitboard to check.
/// * `index` - The index of the bit to check.
///
/// # Returns
///
/// * `true` if the bit at the specified index is set (i.e., 1), otherwise `false`.
///
pub fn bitset(bb: &u64, index: u8) -> bool {
    assert!(index < 64);
    mask(index) & bb != 0
}

pub fn count_pieces(board: &Board) -> i32 {
    let mut result = 0;
    for i in 0..12 {
        for bit in BitIter(board.bitboards[i]) {
            result += 1;
        }
    }
    result
}

/// Generates a bitboard with a single bit set at the specified index.
///
/// # Arguments
///
/// * `index` - The index where the bit is to be set (0-based).
///
/// # Returns
///
/// A `u64` representing the generated bitboard with the bit set at the specified index.
///
/// # Note
///
/// This function generates a bitboard with only one bit set at the given index,
/// while all other bits are zero.
#[allow(dead_code)]
pub fn get_square(index: u8) -> Result<u64, Error> {
    assert!(index < 64, "Index out of range");
    Ok(1 << index)
}
/// Finds the index of the bitboard containing a particular bit.
///
/// This function iterates through the provided `bitboards` and checks each bitboard
/// to find the one containing the specified `index`. If the index is found in any
/// of the bitboards, it returns the index of that bitboard. If the index is out of range
/// (greater than or equal to 64), it will panic.
///
/// # Arguments
///
/// * `bitboards` - A reference to the `Board` struct containing multiple bitboards.
/// * `index` - The index of the bit to find within the bitboards. It should be less than 64.
///
/// # Returns
///
/// * `Some(usize)` - The index of the bitboard containing the specified index.
/// * `None` - If the specified index is not found in any of the bitboards.
///
/// # Panics
///
/// Panics if the specified `index` is out of range (greater than or equal to 64).
///
#[allow(dead_code)]
pub fn find_bitboard(bitboards: &Board, index: u8) -> Option<usize> {
    assert!(index < 64, "Index out of range");
    for (i, bitboard) in bitboards.bitboards.iter().enumerate() {
        if bitset(bitboard, index) {
            return Some(i);
        }
    }
    None
}
#[allow(dead_code)]
pub fn move_piece(bitboard: &mut u64, to_index: u8, from_index: u8) -> () {
    assert!(to_index < 64, "Index out of range");
    assert!(from_index < 64, "Index out of range");
    *bitboard ^= get_square(to_index).unwrap();
    *bitboard ^= get_square(from_index).unwrap();
}

pub struct BitIter(pub u64);

impl BitIter {
    pub fn new(bitboard: u64) -> Self {
        BitIter(bitboard)
    }
}

impl Iterator for BitIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            // Isolate the rightmost 1-bit
            let r = self.0.trailing_zeros();
            // Remove the rightmost 1-bit
            self.0 &= self.0 - 1;
            Some(r)
        }
    }
}

pub fn square_to_algebraic(square: &u8) -> String {
    let file = (square % 8) as u8 + b'a';
    let rank = (7 - (square / 8)) as u8 + b'1';
    format!("{}{}", file as char, rank as char)
}

pub fn algebraic_to_square(algebraic: &str) -> Option<u8> {
    // Check if the input length is exactly 2 characters
    if algebraic.len() != 2 {
        return None; // or handle the error as appropriate
    }

    let chars: Vec<char> = algebraic.chars().collect();
    let file = chars[0] as u8 - b'a'; // Convert 'a'-'h' to 0-7
    let rank = chars[1] as u8 - b'1'; // Convert '1'-'8' to 0-7

    // Calculate the square index
    let square_index = file + 8 * (7 - rank);

    Some(square_index)
}

#[allow(dead_code)]
pub fn draw_bb(bb: u64) {
    println!("     A  B  C  D  E  F  G  H");
    println!("");

    let mut result: String = String::from("");
    // this order is used to preserve little-endian indexing
    for row in (0..8) {
        let k = row * 8;
        let row = 7 - row;
        for col in (k..k + 8)
        //.rev()
        {
            if {
                let mask: u64 = 1 << col;
                mask & bb != 0
            } {
                result.push_str(" 1 ")
            } else {
                result.push_str(" 0 ");
            }
        }
        println!("{}   {result}", row + 1);
        println!("");
        result = String::from("");
    }
}

#[cfg(test)]
mod tests {
    use crate::board::State;

    use super::*;

    #[test]
    fn test_mask() {
        assert_eq!(mask(0), 1);
        assert_eq!(mask(1), 2);
        assert_eq!(mask(7), 128);
    }

    #[test]
    fn test_bitset() {
        let bb: u64 = 0b0000_0000_0000_0001;
        assert_eq!(bitset(&bb, 0), true);
        assert_eq!(bitset(&bb, 1), false);
    }

    #[test]
    fn test_get_square() {
        assert_eq!(get_square(0).unwrap(), 1);
        assert_eq!(get_square(63).unwrap(), 0x8000_0000_0000_0000);
    }

    #[test]
    fn test_find_bitboard() {
        let bitboards = Board {
            bitboards: Box::new([
                0b0000_0000_0000_0001,
                0b0000_0000_0000_0010,
                0b0000_0000_0000_0100,
                0b0000_0000_0000_1000,
                0b0000_0000_0001_0000,
                0b0000_0000_0010_0000,
                0b0000_0000_0100_0000,
                0b0000_0000_1000_0000,
                0b0000_0001_0000_0000,
                0b0000_0010_0000_0000,
                0b0000_0100_0000_0000,
                0b0000_1000_0000_0000,
            ]),
            current_state: State::new(None),
            state_history: vec![State::new(None)],
        };
        assert_eq!(find_bitboard(&bitboards, 0), Some(0));
        assert_eq!(find_bitboard(&bitboards, 1), Some(1));
        assert_eq!(find_bitboard(&bitboards, 2), Some(2));
        assert_eq!(find_bitboard(&bitboards, 3), Some(3));
        assert_eq!(find_bitboard(&bitboards, 4), Some(4));
        assert_eq!(find_bitboard(&bitboards, 5), Some(5));
        assert_eq!(find_bitboard(&bitboards, 6), Some(6));
        assert_eq!(find_bitboard(&bitboards, 7), Some(7));
        assert_eq!(find_bitboard(&bitboards, 8), Some(8));
        assert_eq!(find_bitboard(&bitboards, 9), Some(9));
        assert_eq!(find_bitboard(&bitboards, 10), Some(10));
        assert_eq!(find_bitboard(&bitboards, 11), Some(11));
    }

    #[test]
    fn test_move_piece() {
        let mut bitboard: u64 = 0b0000_0000_0000_0000;
        move_piece(&mut bitboard, 0, 1);
        assert_eq!(bitboard, 0b0000_0000_0000_0011);
    }
}
