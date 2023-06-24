# Collaptz

**Collaptz** is an open computational task, a collaborative verifiable system, to experiment the Collatz conjecture.

## How to run and contribute

```shell
rustup override set nightly # because of zkvm requirement of running nightly rust/cargo.
cargo build
cargo run
```

## Collatz conjecture
A Collatz sequence begins with a positive integer. The sequence follows two simple rules: if the current number is even, it is divided by two; if it is odd, it's tripled and increased by one.

The Collatz conjecture proposes that no matter the starting integer, the sequence will inevitably reach one.


## Methodology
Participants can interact with the WeCollatz system by calling the NFT's `compute` function with their chosen positive integer.
The integer must be within the Ethereum Virtual Machine's `uint` upper limit (2^256).
The `compute` method, in turn, activates an off-chain program using the Bonsai zkVM rollup to continue computing the Collatz sequence.l
This operation generates a receipt that includes a journal and proof of correct computation, all of which is verified through risc0 zkvm.
The updated state of the sequence computation is stored on Filecoin through FilecoinVM, creating a secure and transparent record of the ongoing collaborative computational task.


## Why?
This allows individuals worldwide to contribute to a computational task computing Collatz sequences - permissionless and verified. It features a scalable system. TBD

While the conjecture has been heavily studied, the range of integers we propose have not been fully explored as of mid-2023. A surprise might lie within this computation.


## Implementation notes
We start with a simple algorithm, for the sake of building a proof of concept quickly.
The literature indicates multiple performance optimizations are possible (eg https://doi.org/10.1007/s11227-020-03368-x, precomputation https://en.wikipedia.org/wiki/Collatz\_conjecture#Time%E2%80%93space\_tradeoff, etc). Other proof approaches would probably be a better path to a conclusion.

**potential optimizations**
powers of 2


## "The Collatz Thinker" NFT
This unique NFT will be offered at an auction in late 2023.


