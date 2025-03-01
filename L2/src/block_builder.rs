use alloy::{
    consensus::{Signed, TxEip1559, TxEnvelope},
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    sol,
    sol_types::SolEvent,
};
use alloy_rlp::Decodable;
use futures_util::stream::StreamExt;

sol! {
  contract Inbox {
    event BatchProved(uint256 batchId, bytes[] batchData, uint256 blockNumber);
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
            .await
            .unwrap();

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
                    let Inbox::BatchProved {
                        batchId: _batch_id,
                        batchData,
                        blockNumber,
                    } = log.log_decode().unwrap().inner.data;

                    let transactions: Vec<TxEnvelope> = batchData
                        .iter()
                        .map(|encoded| TxEnvelope::decode(&mut &encoded[..]).unwrap())
                        .collect();

                    let hashes: Vec<String> = transactions
                        .iter()
                        .map(|tx| tx.signature_hash().to_string())
                        .collect();

                    println!("new block 📦");
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
