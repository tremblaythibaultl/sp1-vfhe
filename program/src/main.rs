#![no_main]
sp1_zkvm::entrypoint!(main);

use ttfhe::{
    ggsw::{cmux, GgswCiphertext},
    glwe::GlweCiphertext,
};
pub fn main() {
    // let n = sp1_zkvm::io::read::<u32>();
    // let mut a = 0;
    // let mut b = 1;
    // let mut sum;
    // for _ in 1..n {
    //     sum = a + b;
    //     a = b;
    //     b = sum;
    // }
    // sp1_zkvm::io::write(&a);
    // sp1_zkvm::io::write(&b);

    //let (bsk_i, c_prime, c_prime_rotated): (GgswCiphertext, GlweCiphertext, GlweCiphertext) = env::read();

    let bsk_i = sp1_zkvm::io::read::<GgswCiphertext>();
    let c_prime = sp1_zkvm::io::read::<GlweCiphertext>();
    let c_prime_rotated = sp1_zkvm::io::read::<GlweCiphertext>();

    let res = cmux(&bsk_i, &c_prime, &c_prime_rotated);

    sp1_zkvm::io::write(&res);
}
