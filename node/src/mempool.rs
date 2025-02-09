
use std::fs::File;
use alloy::primitives::TxHash;
use serde_json::to_writer_pretty;

pub fn create_mempool() {
  let transactions: Vec<TxHash> = (0..100)
    .into_iter()  
    .map(|_| {
        let random_bytes: [u8; 32] = rand::random();
        TxHash::from(random_bytes)
    })
    .collect();

  let file = File::create("mempool.json").expect("Failed to create file");
  to_writer_pretty(file, &transactions).expect("Failed to write JSON");
}