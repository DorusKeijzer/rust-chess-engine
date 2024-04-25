pub fn bitset(bb: u64, index: usize) -> bool
{
    let mask: u64 = 1 << index;
    mask & bb != 0
}