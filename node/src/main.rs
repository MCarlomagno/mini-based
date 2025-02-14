mod mempool;
mod monitor;

use monitor::Monitor;
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
  let monitor: Monitor = Monitor::new(l1_inbox_address, rpc_url, event_signature);
  monitor.listen().await;
}
