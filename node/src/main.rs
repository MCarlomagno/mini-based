mod mempool;
mod monitor;

use monitor::Monitor;
use mempool::create_mempool;

#[tokio::main]
async fn main() {
  // creates a static random mempool in mempool.json
  create_mempool();

  // sepolia
  let rpc_url = "wss://ethereum-sepolia-rpc.publicnode.com";
  let l1_inbox_address = "0xdB8eB6D1d24c312DBdd3fDc01B37dD2862D6C391";
  let event_signature = "BatchProved(uint256)";

  // listens new proven batches from L1 inbox.
  let monitor: Monitor = Monitor::new(l1_inbox_address, rpc_url, event_signature);
  monitor.listen_proven_batches().await;
}
