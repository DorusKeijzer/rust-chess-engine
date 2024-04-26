use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::utils;

lazy_static! {
    static ref KNIGHT_MOVES: [u64; 64] = init_knight_tables();
}

fn init_knight_tables() -> [u64;64]
{
    let mut res = [0;64];
    for i in 0..63  
    {
        res[i] = utils::mask(i as u8);
    }
    res
}


fn perft()->()
{
    todo!();
}

