mod mempool;
mod monitor;

use monitor::Monitor;
use mempool::create_mempool;

fn main() {
  // creates a static mempool in mempool.json
  create_mempool();

  // listens new proven batches from L1 inbox.
  let monitor: Monitor = Monitor::new();
  monitor.listen_batches();
}
