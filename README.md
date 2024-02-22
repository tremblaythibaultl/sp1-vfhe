# Verifiable fully homomorphic encryption

This repository hosts a proof of concept for verifiable fully homomorphic encryption.

The aim of this project is to prove and verify the correct execution of a single TFHE bootstrapping.

The branch `verif_enc` allows to generate proofs of correct encryption.

## Getting started

1. [Install sp1](https://succinctlabs.github.io/sp1/getting-started/install.html)
2. Clone the repo
3. Build program `cd program && cargo prove build`
4. Generate proof `cd ../script && RUST_LOG=info cargo run --release` 

Example output:
```
Launching VM at 0
Got plaintext: 3 at 85666
Finished proof verification at 85859
Completed encryption in 85898 ms
```