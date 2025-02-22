# Mini-taiko 🧸

An experimental local implementation of a taiko-like based rollup with total anarchy.

## Components

- L1 contract (inbox) including `proposeBatch` and `proveBatch`
    - sepolia: 0xdB8eB6D1d24c312DBdd3fDc01B37dD2862D6C391
- Sequencer
    - picks txs from mempool, adds to batch & proposes batch.
- Prover
    - runs batches off-chain and submit proofs.
- Node
    - mempool: list of random raw transactions in a json file.
    - monitor: listens to new batches proven on L1 and add them on the chain.

## Getting started

1. Run [anvil](https://book.getfoundry.sh/anvil/) locally
2. Copy funded wallets to fill private keys in `.env`
    - `SEQUENCER_PRIVATE_KEY`
    - `PROVER_PRIVATE_KEY`
    - `INBOX_DEPLOYER_PRIVATE_KEY`
3. Deploy Inbox contract `cd contracts && cargo run`.
4. Open a new tab, run prover, `cd prover && cargo run`.
5. Copy one of the funded wallets private keys and add it to `sequencer/.env` (`PRIVATE_KEY` var)
6. Open a new tab, run sequencer to send a batch of transactions, `cd sequencer && cargo run`.

## Limitations

- Txs are just hashes, not full txs payloads.
- No batch proving, the prover just generates a mock proof that is always valid.
- No L2 consensus or DA, there is a single node that processes and posts all the blocks.

## Future/Missing Work:

- Syncronous composability with L1
- Reorg handling
- Implement proving
- Implement DA layer