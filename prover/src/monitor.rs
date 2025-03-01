use crate::prover::Prover;
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
    event BatchProposed(uint256 batchId, bytes[] batchData);
  }
}

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
                Some(&Inbox::BatchProposed::SIGNATURE_HASH) => {
                    let Inbox::BatchProposed { batchId, batchData } =
                        log.log_decode().unwrap().inner.data;

                    let transactions: Vec<TxEnvelope> = batchData
                        .iter()
                        .map(|encoded|  TxEnvelope::decode(&mut &encoded[..]).unwrap())
                        .collect();

                    let prover = Prover::new(&self.contract_address, &self.rpc_url);
                    prover.prove_batch(batchId, transactions).await;
                }
                _ => (),
            }
        }
    }
}
