# Verifiable fully homomorphic encryption with SP1

This repository hosts a proof of concept for verifiable fully homomorphic encryption.

The aim of this project is to prove and verify the correct execution of a single TFHE bootstrapping.

## Instructions

### Requirements
SP1 zkVM: see installation procedure [here](https://docs.succinct.xyz/docs/sp1/getting-started/install).

### Code structure
The project is divided in a program (code to be executed in the zkVM) and a script (orchestrator for proving and verifying the program).
This follows paradigms introduced by SP1. See [SP1](https://docs.succinct.xyz/docs/sp1/introduction) for more information on the project structure enforced by the zkVM.

The code relies on the [ttfhe](https://github.com/tremblaythibaultl/ttfhe) library, a simple (and thus zkVM-compatible) Rust implementation of the [TFHE](https://eprint.iacr.org/2018/421) cryptosystem. 

### Usage
1. Build the program:
```bash
$ cd program 
$ cargo prove build
```
2. Execute the program:
```bash
$ cd ../script 
$ RUST_LOG=info cargo run --release -- --execute
```

#### WARNING
The results presented in the paper were obtained by running the zkVM on a `hpc7a.96xlarge` AWS EC2 instance with 192 CPU cores and 768 GB memory. 

zkVMs are notoriously resource-hungry. As such, we do not guarantee the reproducibility of our results on less powerful machines. In fact, trying to execute this prototype on a machine with insufficient resources will likely result in a "killed" process.
