# Mini-taiko 🧸

A minimal implementation of a taiko-like based rollup

Components:

- L1 contract (inbox) including `proposeBatch` and `proveBatch`
- Sequencer
    - picks txs from mempool, adds to batch & proposes batch + prover.
- Prover
    - reads batches, “runs” batches and submit proofs.
- Node
    - mempool: a queue of txs with slice & push functions.
    - listens to new batches proven on L1 and add them on the chain.

Limitations

- Txs are just hashes, not full txs payloads.
- No batch proving, the prover just generates a mock proof that is always valid.
- No L2 consensus or DA, there is a single node that processes and posts all the blocks.

Future/Missing Work:

- Support txs payloads + fee mechanism
- Reorg handling
- implement proving + support multiple provers
- Implement DA