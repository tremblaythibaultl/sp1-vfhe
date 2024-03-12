#![no_main]
sp1_zkvm::entrypoint!(main);

use ttfhe::{
    ggsw::{cmux, GgswCiphertext},
    glwe::GlweCiphertext,
};
pub fn main() {
    let bsk_i = sp1_zkvm::io::read::<GgswCiphertext>();
    let c_prime = sp1_zkvm::io::read::<GlweCiphertext>();
    let c_prime_rotated = sp1_zkvm::io::read::<GlweCiphertext>();

    let res = cmux(&bsk_i, &c_prime, &c_prime_rotated);

    sp1_zkvm::io::write(&res);
}
