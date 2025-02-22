mod block_builder;
use block_builder::BlockBuilder;

#[tokio::main]
async fn main() {
  // anvil node.
  let rpc_url = "ws://127.0.0.1:8545";
  let l1_inbox_address = "0xe7f1725e7734ce288f8367e1bb143e90bb3f0512";
  let event_signature = "BatchProved(uint256,bytes[])";

  // listens new proven batches from L1 inbox.
  let block_builder: BlockBuilder = BlockBuilder::new(l1_inbox_address, rpc_url, event_signature);
  block_builder.listen().await;
}
