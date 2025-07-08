#![no_main]
sp1_zkvm::entrypoint!(main);

use ttfhe::{
    ggsw::BootstrappingKey,
    glwe::GlweCiphertext, lwe::LweCiphertext,
};
pub fn main() {
    let bsk_i = sp1_zkvm::io::read::<GgswCiphertext>();
    let c_prime = sp1_zkvm::io::read::<GlweCiphertext>();
    let c_prime_rotated = sp1_zkvm::io::read::<GlweCiphertext>();

    let res_ct = blind_rotated_lut.sample_extract();

    sp1_zkvm::io::write(&res_ct);
}
