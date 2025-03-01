mod monitor;
mod prover;

use monitor::Monitor;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
  dotenv().ok(); 
  let inbox_address = std::env::var("INBOX_ADDRESS").unwrap();
  let l1_port = std::env::var("L1_PORT").unwrap();
  // L1 node.
  let rpc_url = format!("ws://127.0.0.1:{}", l1_port);
  let event_signature = "BatchProposed(uint256,bytes[],uint256)";

  // listens new proven batches from L1 inbox.
  let monitor: Monitor = Monitor::new(&inbox_address, &rpc_url, event_signature);
  monitor.listen().await;
}
