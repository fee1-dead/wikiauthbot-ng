use rand::distr::Distribution;
use rand::distr::slice::Choose;
use rand::rng;
use sha3::{Digest, Sha3_256};

fn main() {
    let alphabet = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    assert_eq!(10 + 26 * 2, alphabet.len());
    let choose = Choose::new(alphabet.as_bytes()).unwrap();
    let s = choose
        .sample_iter(rng())
        .take(32)
        .map(|&u| u as char)
        .collect::<String>();
    let mut hasher = Sha3_256::new();
    hasher.update(&s);
    let x: [u8; 32] = hasher.finalize().into();
    
    println!("key: {s}\nhash: {x:?}");
}
