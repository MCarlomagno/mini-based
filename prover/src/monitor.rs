use alloy::{
  primitives::Address,
  providers::{Provider, ProviderBuilder, WsConnect},
  rpc::types::{BlockNumberOrTag, Filter},
};
use futures_util::stream::StreamExt;

pub struct Monitor {
  contract_address: String,
  event_signature: String,
  rpc_url: String,
}

impl Monitor {
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

    println!("monitoring Inbox contract logs...");

    while let Some(log) = stream.next().await {
      println!("new logs arrived");
      let log_data = log.data();

      println!("Batch proposed: {}", log_data.data);
    }
  }
}