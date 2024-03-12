#![no_main]
sp1_zkvm::entrypoint!(main);

use ttfhe::{
    ggsw::BootstrappingKey,
    glwe::GlweCiphertext, lwe::LweCiphertext,
};
pub fn main() {
    let c = sp1_zkvm::io::read::<LweCiphertext>();
    let bsk = sp1_zkvm::io::read::<BootstrappingKey>();

    let lut = GlweCiphertext::trivial_encrypt_lut_poly();

    let blind_rotated_lut = lut.blind_rotate(c, &bsk);

    let res_ct = blind_rotated_lut.sample_extract();

    sp1_zkvm::io::write(&res_ct);
}
