use alloy::{
  sol,
  consensus::{Signed, TxEip1559},
  primitives::{Address, Bytes, U256},
  providers::{ProviderBuilder, WsConnect}, 
  signers::local::PrivateKeySigner,
  network::EthereumWallet
};
use std::str::FromStr;
use std::time::{Instant, Duration};
use tokio::time::sleep;
use dotenv::dotenv;

sol! {
  #[sol(rpc)]
  contract Inbox {
    function proveBatch(uint256 id, bytes memory proof) public {
        require(_verifyBatch(batches[id], proof), "Invalid proof");
        emit BatchProved(batchId);
    }
  }
}

pub struct Prover {
  contract_address: String,
  rpc_url: String,
}

impl Prover {
  pub fn new(contract_address: &str, rpc_url: &str) -> Self {
    Self {
      rpc_url: rpc_url.to_string(),
      contract_address: contract_address.to_string(),
    }
  }

  pub async fn generate_proof(&self, batch: Vec<Signed<TxEip1559>>) -> Bytes {
    let start = Instant::now();
    println!("New batch detected, starting proof generation... ⏳");
    
    // Simulate proof generation with 10 second delay
    sleep(Duration::from_secs(10)).await;
    
    let duration = start.elapsed();
    println!("Proof generation completed in {:.2?} ✅", duration);
    
    Bytes::from("proof")
  }

  pub async fn prove_batch(&self, batch_id: U256, batch: Vec<Signed<TxEip1559>>) {
    dotenv().ok(); 
    let pk = &std::env::var("PRIVATE_KEY").unwrap();

    let proof = self.generate_proof(batch).await;

    let signer: PrivateKeySigner = PrivateKeySigner::from_str(pk).unwrap();
    let wallet = EthereumWallet::from(signer);
    let inbox_address = Address::from_str(&self.contract_address).unwrap();

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_ws(WsConnect::new(&self.rpc_url))
        .await.unwrap();

    let inbox = Inbox::new(inbox_address, provider);

    let result = inbox
        .proveBatch(batch_id, proof)
        .send().await
        .unwrap();

    println!("Proof submitted to Inbox contract 🚀");
  }
}
