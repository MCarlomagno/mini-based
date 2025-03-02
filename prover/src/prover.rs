use alloy::{
    consensus::TxEnvelope,
    network::EthereumWallet,
    primitives::{Address, Bytes, Uint, U256},
    providers::{ext::AnvilApi, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
};
use std::str::FromStr;
use std::time::{Duration, Instant};
use tokio::time::sleep;

sol! {
  #[sol(rpc)]
  contract Inbox {
    function proveBatch(uint256 id, bytes memory proof, uint256 blockNumber) public {
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

    pub async fn generate_proof(&self, _: Vec<TxEnvelope>) -> Bytes {
        let start = Instant::now();
        println!("New batch detected, starting proof generation... ⏳");

        // Simulate proof generation with 10 second delay
        sleep(Duration::from_secs(10)).await;

        let duration = start.elapsed();
        println!("Proof generation completed in {:.2?} ✅", duration);

        Bytes::from("proof")
    }

    pub async fn prove_batch(&self, batch_id: U256, batch: Vec<TxEnvelope>, block_number: U256) {
        // setup a random prover wallet for sequencing to L1
        let prover_key: PrivateKeySigner = PrivateKeySigner::random();
        let prover_wallet = EthereumWallet::from(prover_key);

        let proof = self.generate_proof(batch).await;

        let inbox_address = Address::from_str(&self.contract_address).unwrap();

        let provider = ProviderBuilder::new()
            .wallet(&prover_wallet)
            .on_builtin(&self.rpc_url)
            .await
            .unwrap();

        let balance: Uint<256, 4> = Uint::from(100_000_000_000_000_000_000u128);
        provider
            .anvil_set_balance(prover_wallet.default_signer().address(), balance)
            .await
            .unwrap();
        provider
            .anvil_impersonate_account(prover_wallet.default_signer().address())
            .await
            .unwrap();

        let inbox = Inbox::new(inbox_address, provider);

        let _ = inbox
            .proveBatch(batch_id, proof, block_number)
            .send()
            .await
            .unwrap();

        println!("Proof submitted to Inbox contract 🚀");
    }
}
