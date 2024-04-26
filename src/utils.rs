use std::{error, fmt::Error};

use crate::board::Board;

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
    let mask: u64 = 1 << index;
    println!("{}", mask);
    mask & bb != 0
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

pub fn move_piece(bitboard: &mut u64, to_index: u8, from_index: u8) -> ()
{
    assert!(to_index < 64, "Index out of range");
    assert!(from_index < 64, "Index out of range");
    *bitboard ^= get_square(to_index).unwrap();
    *bitboard ^= get_square(from_index).unwrap();
}
