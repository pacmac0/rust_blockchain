/*
Credit:
https://github.com/GeekLaunch/blockchain-rust/blob/master
*/

use blockchainlib::*;

fn main() {
    // Consts.
    // TODO: make difficulty adjustable
    let difficulty = 0x0000ffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new( 
        0, 
        now(), 
        vec![0; 32], 
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 50,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 50,
                    },
                ],
            },
        ], 
        difficulty 
    );
    genesis_block.mine();
    println!("{:?}", &genesis_block);
    let mut last_blocks_hash = genesis_block.block_hash.clone();

    let mut blockchain = Blockchain::new();
    blockchain.update_add_block(genesis_block).expect("Failing to add genesis block to blockchain");

    // add second block to the chain
    let mut block = Block::new( 
        1, 
        now(), 
        last_blocks_hash,
        vec![
            Transaction{ // coinbase transaction
                inputs: vec![],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Miner".to_owned(),
                        value: 50,
                    },
                ],
            },
            Transaction{ // user transaction
                // TODO uses hard coded values for idx etc. (write "new" function for transactions)
                inputs: vec![
                    blockchain.blocks[0].transactions[0].outputs[0].clone(),
                ],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 40,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 5,
                    },
                ],
            },
        ],
        difficulty 
    );

    block.mine();
    // debug
    println!("{:?}", &block);
    
    last_blocks_hash = block.block_hash.clone();

    // add block to chain
    blockchain.update_add_block(block).expect("Blockchain corrupted");
}