mod block_builder;

use std::str::FromStr;
use alloy::node_bindings::Anvil;
use block_builder::BlockBuilder;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
  dotenv().ok();
  // run node
  let inbox_address = std::env::var("INBOX_ADDRESS").unwrap();
  let l1_port_str = std::env::var("L1_PORT").unwrap();
  let l2_port_str = std::env::var("L2_PORT").unwrap();
  let l2_port = u16::from_str(&l2_port_str).unwrap();
  let anvil = Anvil::new().port(l2_port).block_time(1).try_spawn().unwrap();
  
  println!("running L2 node in port {:?} 🛜", anvil.port());

  // anvil L1 node.
  let rpc_url = format!("ws://127.0.0.1:{}", l1_port_str);
  let event_signature = "BatchProved(uint256,bytes[])";

  // listens new proven batches from L1 inbox.
  let block_builder: BlockBuilder = BlockBuilder::new(&inbox_address, &rpc_url, event_signature);
  block_builder.listen().await;
}
