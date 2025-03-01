# Mini-based 🧸

An experimental implementation of a total anarchy based rollup

## Components

- L1: Anvil node where L1 inbox contract is deployed.
- L2: Anvil node + monitor L1 inbox to construct new valid blocks.
- Sequencer: generates random valid L2 txs (analogous to a mempool) & submits batches to L1 inbox.
- Prover: Monitors proposed batches and runs them off-chain, then submits proofs.


## Getting started

1. Run L1 and L2 locally in different tabs
```sh
cd L1 && cargo run
cd L2 && cargo run
```
2. Copy funded wallets to fill private keys in `.env`
    - `PROVER_PRIVATE_KEY`
    - `L1_PORT`
    - `L2_PORT`
3. Open a new tab, run prover, `cd prover && cargo run`.
4. Open a new tab, run sequencer to send a batch of transactions, `cd sequencer && cargo run`.

## Limitations

- No DA
- No batch proving
- No Syncronous composability with L1 or L2s
