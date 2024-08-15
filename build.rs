#[path = "src/primes.rs"]
mod primes;

use primes::Primes;
use std::fs;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let primes = Primes::build(5_000_000);
    println!("Found {} primes", primes.primes().len());

    fs::write("assets/primes.bin", bytemuck::cast_slice(primes.primes())).unwrap();
}
