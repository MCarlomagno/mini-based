use alloy::{
  primitives::{keccak256, Address},
  providers::{Provider, ProviderBuilder, WsConnect},
  rpc::types::{BlockNumberOrTag, Filter}, sol, sol_types::SolEvent,
};
use futures_util::stream::StreamExt;

sol! {
  contract Inbox {
    event BatchProved(uint256 batchId, bytes batchData);
  }
}

pub struct BlockBuilder {
  contract_address: String,
  event_signature: String,
  rpc_url: String,
}

impl BlockBuilder {
  pub fn new(contract_address: &str, rpc_url: &str, event_signature: &str) -> Self {
    Self {
      rpc_url: rpc_url.to_string(),
      contract_address: contract_address.to_string(),
      event_signature: event_signature.to_string(),
    }
  }

  pub async fn listen(&self) {
    let provider = ProviderBuilder::new()
      .on_ws(WsConnect::new(&self.rpc_url))
      .await.unwrap();

    let filter = Filter::new()
      .address(self.contract_address.parse::<Address>().unwrap())
      .event(&self.event_signature)
      .from_block(BlockNumberOrTag::Latest);

    let sub = provider.subscribe_logs(&filter).await.unwrap();
    let mut stream = sub.into_stream();

    println!("⏳ monitoring Inbox contract logs...");

    while let Some(log) = stream.next().await {
      match log.topic0() {
        Some(&Inbox::BatchProved::SIGNATURE_HASH) => {
          let Inbox::BatchProved { batchId, batchData } =
              log.log_decode().unwrap().inner.data;

          let batch_hex = hex::encode(&batchData);
          let raw_transactions: Vec<String> = batch_hex
              .split("f8b00")
              .filter(|s| !s.is_empty())
              .map(|tx| format!("0xf8b00{}", tx))
              .collect();

          let hashes: Vec<String> = raw_transactions
            .iter()
            .map(|tx| {
              let hash = keccak256(tx);
              format!("0x{:x}", hash)
            })
            .collect();

            println!("new block 📦");
            println!("Transaction hashes:");
            for hash in &hashes {
                println!("├─ {}", hash);
            }
            println!("└──────────────────────────────────────────");
        }
        _ => (),
      }
    }
  }
}