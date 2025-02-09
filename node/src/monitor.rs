use alloy::primitives::TxHash;

pub struct Transaction {
  hash: TxHash,
}

struct Block {
  transactions: Vec<Transaction>,
}

pub struct Monitor {
  blocks: Vec<Block>,
}

impl Monitor {

  pub fn new() -> Self {
    Self {
        blocks: Vec::new(),
    }
  }

  pub fn listen_batches(&self) {
    // TODO: listen proven batches emmited by 
    // Inbox contract and create blocks from it.
  }
}