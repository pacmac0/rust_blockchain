/*
Credit:
https://github.com/GeekLaunch/blockchain-rust/blob/master/src/lib.rs
*/

use blockchainlib::*;

fn main() {
    let genesis_block = Block::new( 2, now(), vec![0; 32], "genesis_transaction".to_owned(), 0, 0 );
    println!("{:?}", &genesis_block);

    let hash = genesis_block.hash();
    println!("{:?}", &hash);
}