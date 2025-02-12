# Mini-taiko 🧸

A minimal implementation of a taiko-like based rollup

Components:

- L1 contract (inbox) including `proposeBatch` and `proveBatch`
    - sepolia: 0xdB8eB6D1d24c312DBdd3fDc01B37dD2862D6C391
- Sequencer
    - picks txs from mempool, adds to batch & proposes batch.
- Prover
    - runs batches off-chain and submit proofs.
- Node
    - mempool: list of random raw transactions in a json file.
    - monitor: listens to new batches proven on L1 and add them on the chain.

Limitations

- Txs are just hashes, not full txs payloads.
- No batch proving, the prover just generates a mock proof that is always valid.
- No L2 consensus or DA, there is a single node that processes and posts all the blocks.

Future/Missing Work:

- Support txs payloads + fee mechanism
- Reorg handling
- implement proving + support multiple provers
- Implement DA