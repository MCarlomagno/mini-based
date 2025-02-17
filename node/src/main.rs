mod mempool;
mod block_builder;

use block_builder::BlockBuilder;
use mempool::create_mempool;

#[tokio::main]
async fn main() {
  // creates a static random mempool in mempool.json
  create_mempool();

  // anvil node.
  let rpc_url = "ws://127.0.0.1:8545";
  let l1_inbox_address = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
  let event_signature = "BatchProved(uint256)";

  // listens new proven batches from L1 inbox.
  let block_builder: BlockBuilder = BlockBuilder::new(l1_inbox_address, rpc_url, event_signature);
  block_builder.listen().await;
}
