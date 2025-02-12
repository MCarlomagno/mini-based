
use alloy::consensus::{transaction::RlpEcdsaTx, TxEip1559};
use alloy::hex;
use alloy::primitives::{Address, PrimitiveSignature, B256, U256};
use alloy::rpc::types::AccessList;
use rand::{Rng, SeedableRng, rngs::StdRng};

use serde_json;
use std::fs::File;
use std::io::Write;

fn generate_random_raw_transaction(rng: &mut StdRng) -> String {
    let sig = PrimitiveSignature::from_scalars_and_parity(
      B256::random(),
      B256::random(),
      false,
    );

    let tx = TxEip1559 {
      chain_id: 1,
      nonce: rng.gen_range(1..100),
      input: hex!("a22cb4650000000000000000000000005eee75727d804a2b13038928d36f8b188945a57a0000000000000000000000000000000000000000000000000000000000000000").into(),
      gas_limit: 21000,
      to: Address::random().into(),
      value: U256::from(0_u64),
      max_fee_per_gas: 0x4a817c800,
      max_priority_fee_per_gas: 0x3b9aca00,
      access_list: AccessList::default(),
    };
    let mut buf = vec![];
    tx.rlp_encode_signed(&sig, &mut buf);
    
    format!("0x{}", hex::encode(buf))
}

pub fn create_mempool() {
    let mut rng = StdRng::from_entropy();
    let num_transactions = 100;
    let transactions: Vec<String> = (0..num_transactions)
        .map(|_| generate_random_raw_transaction(&mut rng))
        .collect();

    // Save to JSON file
    let json_data = serde_json::to_string_pretty(&transactions).unwrap();
    let mut file = File::create("mempool.json").expect("Failed to create file");
    file.write_all(json_data.as_bytes()).expect("Failed to write to file");

    println!("✅ Generated {} random transactions and saved to mempool.json", num_transactions);
}