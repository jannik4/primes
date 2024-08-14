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

// use std::collections::HashSet;
//
// #[derive(Debug, Clone, Resource)]
// pub struct Primes {
//     numbers: Vec<Divisors>,
// }
//
// impl Primes {
//     pub fn build(count: usize) -> Self {
//         let mut numbers = vec![Divisors(HashSet::new()); count];
//
//         for i in 0..count {
//             numbers[i] = Divisors::build(i as u64, &numbers[..i]);
//         }
//
//         Self { numbers }
//     }
//
//     pub fn primes(&self) -> impl Iterator<Item = u64> + '_ {
//         self.numbers.iter().enumerate().filter_map(|(i, divisors)| {
//             if divisors.0.len() == 2 {
//                 Some(i as u64)
//             } else {
//                 None
//             }
//         })
//     }
// }
//
// #[derive(Debug, Clone)]
// pub struct Divisors(HashSet<u64>);
//
// impl Divisors {
//     fn build(n: u64, other: &[Divisors]) -> Self {
//         if n == 0 {
//             return Self(HashSet::new());
//         }
//
//         let mut divisors = HashSet::new();
//         divisors.insert(1);
//         divisors.insert(n);
//
//         let mut x = n;
//         let mut d = n - 1; // TODO: start from floor(sqrt(n))
//         while d > 1 {
//             if x % d == 0 {
//                 divisors.extend(other[d as usize].0.iter().cloned());
//
//                 x /= d;
//                 d = x - 1;
//             } else {
//                 d -= 1;
//             }
//         }
//
//         Self(divisors)
//     }
// }
