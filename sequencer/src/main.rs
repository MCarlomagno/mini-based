use alloy::{
    eips::Encodable2718,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, Bytes, Uint, U256},
    providers::{ext::AnvilApi, Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};
use dotenv::dotenv;
use std::str::FromStr;

sol! {
    #[sol(rpc)]
    contract Inbox {
      // anyone can propose a batch
      function proposeBatch(bytes[] calldata batchData, uint256 blockNumber) public {
          batches[batchId] = batchData;
          emit BatchProposed(batchId, batchData);
          batchId++;
      }
    }
}

async fn create_random_batch(sender: &EthereumWallet, chain_id: u64) -> Vec<Vec<u8>> {
    let num_transactions = 10;
    let mut batch = Vec::new();
    let mut nonce = 11234;

    // Use a for loop to build transactions one by one
    for _ in 0..num_transactions {
        let tx = TransactionRequest::default()
            .with_from(sender.default_signer().address())
            .with_nonce(nonce)
            .with_to(Address::random())
            .with_value(U256::from(100))
            .with_gas_limit(21_000)
            .with_max_priority_fee_per_gas(1_000_000_000)
            .with_max_fee_per_gas(20_000_000_000)
            .with_chain_id(chain_id)
            .build(&sender)
            .await
            .unwrap();

        nonce += 1;
        let mut buf = vec![];
        tx.encode_2718(&mut buf);
        batch.push(buf);
    }

    batch
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // load L1 chain provider.
    let inbox_address = std::env::var("INBOX_ADDRESS").unwrap();
    let l1_port_str = std::env::var("L1_PORT").unwrap();
    let l2_port_str = std::env::var("L2_PORT").unwrap();
    let l1_rpc_url = format!("http://localhost:{}", l1_port_str);
    let l2_rpc_url = format!("http://localhost:{}", l2_port_str);
    let l1_provider = ProviderBuilder::new()
        .on_builtin(&l1_rpc_url)
        .await
        .unwrap();
    let l2_provider = ProviderBuilder::new()
        .on_builtin(&l2_rpc_url)
        .await
        .unwrap();

    println!("Setting up sequencer wallet ⏳");
    // setup a random sequencer wallet for sequencing to L1
    let sequencer_key: PrivateKeySigner = PrivateKeySigner::random();
    let sequencer_wallet = EthereumWallet::from(sequencer_key);
    let balance: Uint<256, 4> = Uint::from(100_000_000_000_000_000_000u128);
    l1_provider
        .anvil_set_balance(sequencer_wallet.default_signer().address(), balance)
        .await
        .unwrap();
    l1_provider
        .anvil_impersonate_account(sequencer_wallet.default_signer().address())
        .await
        .unwrap();

    println!("Setting up L2 sender wallet ⏳");
    // setup a random tx sender wallet for constructing L2 batches.
    let sender_key: PrivateKeySigner = PrivateKeySigner::random();
    let sender_wallet = EthereumWallet::from(sender_key);
    let balance: Uint<256, 4> = Uint::from(100_000_000_000_000_000_000u128);
    l2_provider
        .anvil_set_balance(sender_wallet.default_signer().address(), balance)
        .await
        .unwrap();
    let l2_chain_id = l2_provider.get_chain_id().await.unwrap();

    println!("Creating random batch of transactions ⏳");
    let batch = create_random_batch(&sender_wallet, l2_chain_id).await;

    // sends batch to L1 Inbox.
    let inbox_address = Address::from_str(&inbox_address).unwrap();
    let inbox = Inbox::new(inbox_address, &l1_provider);

    println!("Sending transactions to mempool ⏳");
    let mut batch_as_bytes = Vec::new();
    for tx in batch.clone() {
        let _ = l2_provider.send_raw_transaction(&tx).await.unwrap();
        batch_as_bytes.push(Bytes::from(tx));
    }

    println!("Proposing batch to L1 ⏳");
    let block_number = l2_provider.get_block_number().await.unwrap();
    let _ = inbox
        .proposeBatch(batch_as_bytes, Uint::from(block_number))
        .from(sequencer_wallet.default_signer().address())
        .send()
        .await
        .unwrap();

    println!("transactions sent");
}
