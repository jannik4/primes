#![allow(dead_code)] // module is used in this crate and in the build script

#[derive(Debug, Clone)]
pub struct Primes {
    primes: Vec<u32>,
}

impl Primes {
    pub fn build(max: u32) -> Self {
        Self {
            primes: (0..=max).filter(|n| is_prime(*n)).collect(),
        }
    }

    pub fn from_unchecked(primes: Vec<u32>) -> Self {
        Self { primes }
    }

    pub fn primes(&self) -> &[u32] {
        &self.primes
    }
}

fn is_prime(n: u32) -> bool {
    let limit = (n as f64).sqrt() as u32;
    n >= 2 && (2..=limit).all(|i| n % i != 0)
}
