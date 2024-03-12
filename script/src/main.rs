//! A simple script to generate and verify the proof of a given program.

use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
use std::time::Instant;
use ttfhe::{
    ggsw::compute_bsk,
    glwe::keygen,
    lwe::{lwe_keygen, LweCiphertext},
    utils::{encode, decode_bootstrapped},
};
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.

    let sk1 = lwe_keygen();
    let sk2 = keygen();
    let bsk = compute_bsk(&sk1, &sk2); // list of encryptions under `sk2` of the bits of `sk1`.

    let c = LweCiphertext::encrypt(encode(2), &sk1).modswitch(); // "noisy" ciphertext that will be bootstrapped

    let now = Instant::now();

    let mut stdin = SP1Stdin::new();
    stdin.write(&c);
    stdin.write(&bsk);

    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read output.
    let res_ct = proof.stdout.read::<LweCiphertext>();

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    let res_pt = decode_bootstrapped(res_ct.decrypt(&sk1));
    println!("Computed bootstrapping and got plaintext {} in {}", res_pt, now.elapsed().as_secs());
}
