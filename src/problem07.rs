#![allow(dead_code)]
use crate::prime::{Primes, sieve_of_eratosthenes};

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is
// 13.
//
// What is the 10_001st prime number?
fn problem(num: usize) -> u64 {
    let primes = sieve_of_eratosthenes(105_000);
    primes.get(num - 1).unwrap().to_owned()
}

fn brute_force(num: u64) -> u64 {
    let mut primes = Primes::new();
    let mut prime_count = 0;
    let mut natural_count = 2;

    loop {
        if primes.is_prime(natural_count) {
            prime_count += 1;
            
            if prime_count== num {
                return natural_count
            }
        }

        natural_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    #[test]
    fn problem_test() {
        let now = Instant::now();
        assert_eq!(problem(10_001), 104_743);
        println!("{}", now.elapsed().as_millis());
    }

    #[test]
    #[ignore]
    fn brute_force_test() {
        let now = Instant::now();
        assert_eq!(brute_force(10_001), 104_743);
        println!("{}", now.elapsed().as_millis());
    }
}
