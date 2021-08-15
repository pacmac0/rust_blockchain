/*
Credit:
https://github.com/GeekLaunch/blockchain-rust/blob/master/src/lib.rs
*/

use blockchainlib::*;

fn main() {
    // Consts.
    let difficulty = 0x0000ffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new( 
        0, 
        now(), 
        vec![0; 32], 
        "genesis_transaction".to_owned(), 
        0, 
        difficulty 
    );
    genesis_block.mine();
    println!("{:?}", &genesis_block);
    let mut last_blocks_hash = genesis_block.block_hash.clone();

    let mut blockchain = Blockchain::new(genesis_block);

    // add number of blocks to the chain(Vec.)
    for i in 1..=10 {
        let mut block = Block::new( 
            i, 
            now(), 
            last_blocks_hash,
            "blocks_transactions".to_owned(),
            0, 
            difficulty 
        );

        block.mine();
        // debug
        println!("{:?}", &block);
        
        last_blocks_hash = block.block_hash.clone();
        blockchain.blocks.push(block);

        // test simple verification function for chain
        if blockchain.verify_chain() {
            println!("Chain is valide");
        } else {
            println!("Chain got corrupted!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        }
        
        
    }
}