use bevy::prelude::*;

#[derive(Debug, Clone, Resource)]
pub struct Primes {
    primes: Vec<u32>,
}

impl Primes {
    pub fn build(max: u32) -> Self {
        Self {
            primes: (0..=max).filter(|n| is_prime(*n)).collect(),
        }
    }

    pub fn primes(&self) -> impl Iterator<Item = u32> + '_ {
        self.primes.iter().cloned()
    }
}

fn is_prime(n: u32) -> bool {
    let limit = (n as f64).sqrt() as u32;
    n >= 2 && (2..=limit).all(|i| n % i != 0)
}
