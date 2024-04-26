use std::{error, fmt::Error};

use crate::board::Board;

/// Creates a mask from a given index  
pub fn mask(index: u8) -> u64 { 1 << index }

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
pub fn bitset(bb: &u64, index: u8) -> bool
{
    assert!(index < 64);
    mask(index) & bb != 0
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

pub fn get_square(index: u8) -> Result<u64, Error> 
{
    assert!(index < 64, "Index out of range");
    Ok(1 << index)
}
#[allow(dead_code)]
/// Searches for the presence of a specific bit at the given index across multiple bitboards.
///
/// # Arguments
///
/// * `bitboards` - A collection of bitboards to search within.
/// * `index` - The index of the bit to search for (0-based).
///
/// # Returns
///
/// * `Some(u64)` containing the first bitboard where the specified bit is set, if found.
/// * `None` if the specified bit is not set in any of the bitboards.
///
/// # Note
///
/// This function assumes that each bitboard represents a set of bits and `index` refers to
/// a position within these bitboards.
pub fn find_bitboard(bitboards: &Board, index: u8) -> Option<usize> {
    assert!(index < 64, "Index out of range");
    for (i, bitboard) in bitboards.bitboards.iter().enumerate() {
        println!("{},bb: {}, index: {}", i, *bitboard, index);
        if bitset(bitboard, index) {
            return Some(i);
        }
    }
    None
}
#[allow(dead_code)]
pub fn move_piece(bitboard: &mut u64, to_index: u8, from_index: u8) -> ()
{
    assert!(to_index < 64, "Index out of range");
    assert!(from_index < 64, "Index out of range");
    *bitboard ^= get_square(to_index).unwrap();
    *bitboard ^= get_square(from_index).unwrap();
}


#[allow(dead_code)]
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


#[cfg(test)]
mod tests {
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
            bitboards: &mut [
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
            ],
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

    // Add more tests as needed for other functions...
}
