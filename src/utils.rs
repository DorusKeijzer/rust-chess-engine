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
/// # Examples
///
/// ```
/// let bb: u64 = 0b1010;
/// assert_eq!(bitset(bb, 1), false); // Bit at index 1 is not set
/// assert_eq!(bitset(bb, 3), true);  // Bit at index 3 is set
/// ```
pub fn bitset(bb: u64, index: usize) -> bool
{
    let mask: u64 = 1 << index;
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
/// # Examples
///
/// ```
/// use your_crate_name::get_square;
///
/// let index = 3;
/// let square = get_square(index);
/// assert_eq!(square, 0b0000_1000);
/// ```
///
/// # Note
///
/// This function generates a bitboard with only one bit set at the given index,
/// while all other bits are zero.
#[allow(dead_code)]

pub fn get_square(index: u64) -> Result<u64, Error> 
{
    if index < 63 { return Err(Error)}
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
/// # Examples
///
/// ```
/// use your_crate_name::find_bitboard;
///
/// let bitboards = your_crate_name::Board { /* initialize bitboards */ };
/// let index = 10;
///
/// match find_bitboard(bitboards, index) {
///     Some(bitboard) => println!("Bitboard with the bit set at index {}: {}", index, bitboard),
///     None => println!("Bit not found at index {}", index),
/// }
/// ```
///
/// # Note
///
/// This function assumes that each bitboard represents a set of bits and `index` refers to
/// a position within these bitboards.
pub fn find_bitboard(bitboards: Board, index: usize) -> Option<u64>
{
    if index > 63 { return None }
    for bitboard in bitboards.bitboards
    {
        if bitset(bitboard, index){ return Some(bitboard) }
    }
    return None
}