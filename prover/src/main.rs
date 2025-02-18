mod monitor;
mod prover;

use monitor::Monitor;

#[tokio::main]
async fn main() {
  // anvil node.
  let rpc_url = "ws://127.0.0.1:8545";
  let l1_inbox_address = "0x5fc8d32690cc91d4c39d9d3abcbd16989f875707";
  let event_signature = "BatchProposed(uint256,bytes)";

  // listens new proven batches from L1 inbox.
  let monitor: Monitor = Monitor::new(l1_inbox_address, rpc_url, event_signature);
  monitor.listen().await;
}
