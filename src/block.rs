#![allow(unused)]

use std::collections::LinkedList as List;

struct BlockChain {
    blocks: List<Block>
}

struct Block {
    hash: String,
    id: u128,
    transactions: List<Transaction>,
}

struct Transaction {
    inputs: List<TxIn>,
    outputs: List<TxOut>,
    txid: String,
}

struct TxIn {
    prev_txid: String,
    out: usize,
    signature: String, // to spend the output
}

struct TxOut {
    public_address: String,
    satoshis: u64, 
    // 1 btc = 10^8 satoshis, in total 10^8 * 21 * 10^6 = 2.1 * 10^15
    // maximum value of u64 is greater than 10^19
    // so u64 is enough to store all valid satoshis
}

impl BlockChain {
    fn new() -> Self {
        BlockChain {
            blocks: List::new(),
        }
    }
    
    pub fn add_new_block(&mut self, block: Block) {
        self.blocks.push_back(block);
    }
    
}

impl Block {
    fn new(hash: String, id: u128) -> Self {
        Block {
            hash,
            id,
            transactions: List::new()
        }
    }
    
    pub fn add_transactions(&mut self, transaction: Transaction) {
        self.transactions.push_back(transaction);
    }
}

impl Transaction {
    fn new(txid: String) -> Self {
        Transaction { 
            inputs: List::new(), 
            outputs: List::new(), 
            txid
        }
    }
}

impl TxIn {
    fn new(prev_txid: String, out: usize, signature: String) -> Self {
        TxIn { 
            prev_txid, 
            out, 
            signature 
        }
    }
}

impl TxOut {
    fn new(public_address: String, satoshis: u64) -> Self {
        TxOut { 
            public_address, 
            satoshis
        }
    }
}

