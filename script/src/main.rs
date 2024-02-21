//! A simple script to generate and verify the proof of a given program.

use rand_distr::{Distribution, Normal};
use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use std::time::Instant;
use ttfhe::{
    glwe::{keygen, GlweCiphertext},
    utils::decode,
};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let sk = keygen();

    let msg = 3u8; // should be in 0..16

    // pick random noise
    let sigma = f32::powf(2.0, 7.0);
    let normal = Normal::new(0.0, sigma).unwrap();
    let e = normal.sample(&mut rand::thread_rng()).round() as i32;

    // start timer
    let now = Instant::now();

    let mut stdin = SP1Stdin::new();
    // write secret key, noise and plaintext to VM input
    stdin.write(&sk);
    stdin.write(&e);
    stdin.write(&msg);

    // launch VM proving process
    println!("Launching VM at {}", now.elapsed().as_millis());
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // read output
    let ciphertext: GlweCiphertext = proof.stdout.read::<GlweCiphertext>();
    println!(
        "Got plaintext: {} at {}",
        decode(ciphertext.decrypt(&sk)),
        now.elapsed().as_millis()
    );

    // verify proof
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    println!(
        "Finished proof verification at {}",
        now.elapsed().as_millis()
    );

    // save proof (9.7M)
    proof
        .save(&"proof-with-io.json")
        .expect("saving proof failed");

    println!("Completed encryption in {} ms", now.elapsed().as_millis());
}
