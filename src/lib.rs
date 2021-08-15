type Hash = Vec<u8>;
type Address = String; // as string for readability, should become public address(hash)

use std::time::{ SystemTime, UNIX_EPOCH };

mod block;
pub use crate::block::Block;
mod hashable;
pub use crate::hashable::Hashable;
mod blockchain;
pub use crate::blockchain::Blockchain;
pub mod transaction;
pub use crate::transaction::Transaction;

pub fn now () -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

pub fn u32_bytes (u: &u32) -> [u8; 4] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
    ]
}

pub fn u64_bytes (u: &u64) -> [u8; 8] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
    ]
}

pub fn u128_bytes (u: &u128) -> [u8; 16] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
        (u >> 8 * 0x8) as u8,
        (u >> 8 * 0x9) as u8,
        (u >> 8 * 0xa) as u8,
        (u >> 8 * 0xb) as u8,
        (u >> 8 * 0xc) as u8,
        (u >> 8 * 0xd) as u8,
        (u >> 8 * 0xe) as u8,
        (u >> 8 * 0xf) as u8,
    ]
}

pub fn difficulty_bytes_as_u128 (vec: &Vec<u8>) -> u128 {
    ((vec[31] as u128) << 0xf * 8) |
    ((vec[30] as u128) << 0xe * 8) |
    ((vec[29] as u128) << 0xd * 8) |
    ((vec[28] as u128) << 0xc * 8) |
    ((vec[27] as u128) << 0xb * 8) |
    ((vec[26] as u128) << 0xa * 8) |
    ((vec[25] as u128) << 0x9 * 8) |
    ((vec[24] as u128) << 0x8 * 8) |
    ((vec[23] as u128) << 0x7 * 8) |
    ((vec[22] as u128) << 0x6 * 8) |
    ((vec[21] as u128) << 0x5 * 8) |
    ((vec[20] as u128) << 0x4 * 8) |
    ((vec[19] as u128) << 0x3 * 8) |
    ((vec[18] as u128) << 0x2 * 8) |
    ((vec[17] as u128) << 0x1 * 8) |
    ((vec[16] as u128) << 0x0 * 8)
}