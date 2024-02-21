#![no_main]
sp1_zkvm::entrypoint!(main);

use ttfhe::{
    glwe::{GlweCiphertext, SecretKey},
    k,
    poly::ResiduePoly,
    utils::encode,
};
pub fn main() {
    let sk = sp1_zkvm::io::read::<SecretKey>();
    let e = sp1_zkvm::io::read::<i32>();
    let msg = sp1_zkvm::io::read::<u8>();

    assert!(msg < 15);
    assert!(e.abs() < 100000);

    let mu = encode(msg);

    let mu_star: u32 = mu.wrapping_add_signed(e);

    let mask: Vec<ResiduePoly> = (0..k).map(|_| ResiduePoly::get_random()).collect();

    let mut body = ResiduePoly::default();
    for i in 0..k {
        body.add_assign(&mask[i].mul(&sk.polys[i]));
    }

    body.add_constant_assign(mu_star as u32);

    let c = GlweCiphertext { mask, body };

    sp1_zkvm::io::write(&c);
}
