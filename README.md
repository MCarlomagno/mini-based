# Mini-based 🧸

An experimental implementation of a total anarchy based rollup.

## Components

- L1: Anvil node where L1 inbox contract is deployed.
- L2: Anvil node + monitor L1 inbox to construct new valid blocks.
- Sequencer: generates random valid L2 txs (analogous to a mempool) & submits batches to L1 inbox.
- Prover: Monitors proposed batches and runs them off-chain, then submits proofs.


## Getting started
1. Define ports for L1 and L2 anvil nodes and add them to your `.env`
```
L1_PORT=8545
L2_PORT=8546
```
1. Run L1 and L2 in different terminal tabs
```sh
cd L1 && cargo run
cd L2 && cargo run
```
2. After running L1, you should see the inbox address in the logs, add it as env variable
```sh
INBOX_ADDRESS=0x...
```
3. Open a new terminal tab, run prover, `cd prover && cargo run`.
4. Open a new tab, run sequencer to send a batch of transactions, `cd sequencer && cargo run`.

## Limitations

- No batch proving (yet)
- No Syncronous composability with L1 or L2s (yet)
