//! A simple script to generate and verify the proof of a given program.

use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use std::time::Instant;
use ttfhe::{
    ggsw::{compute_bsk, BootstrappingKey},
    glwe::{keygen, GlweCiphertext},
    lwe::{lwe_keygen, LweCiphertext},
    utils::encode,
};
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate key material
    let sk1 = lwe_keygen();
    let sk2 = keygen();
    let bsk = compute_bsk(&sk1, &sk2); // list of encryptions under `sk2` of the bits of `sk1`.

    let c = LweCiphertext::encrypt(encode(2), &sk1).modswitch(); // "noisy" ciphertext that will be bootstrapped

    step_by_step_blind_rotation(&c, &bsk);
}

fn step_by_step_blind_rotation(c: &LweCiphertext, bsk: &BootstrappingKey) {
    let mut c_prime = GlweCiphertext::trivial_encrypt_lut_poly();

    // multiply by X^-b over the polynomial ring
    c_prime.body = c_prime.body.multiply_by_monomial((2048 - c.body) as usize);

    for i in 0..bsk.len() {
        let now = Instant::now();

        // Write the inputs to the environment.
        let mut stdin = SP1Stdin::new();
        stdin.write(&bsk[i]);
        stdin.write(&c_prime);
        stdin.write(&c_prime.rotate(c.mask[i]));

        // Generate the proof.
        let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

        // Read output.
        c_prime = proof.stdout.read::<GlweCiphertext>();

        // Verify proof.
        SP1Verifier::verify(ELF, &proof).expect("verification failed");

        // Save proof.
        proof
            .save(&format!("proof-with-io_{}.json", i))
            .expect("saving proof failed");

        println!("succesfully generated and verified proof for the program!");

        println!(
            "Computed blind rotation step number {i} in {} seconds",
            now.elapsed().as_secs()
        );
    }
}
