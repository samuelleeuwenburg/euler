#![allow(dead_code)]
use crate::prime::{Primes};

// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?
fn problem(num: u64) -> u64 {
    let mut primes = Primes::new();
    let prime_factors = primes.get_prime_factorization(num);
    prime_factors.last().cloned().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_test() {
        assert_eq!(problem(12), 3);
        assert_eq!(problem(42), 7);
        assert_eq!(problem(128), 2);
        assert_eq!(problem(60100), 601);
        assert_eq!(problem(600_851_475_143), 6857);
    }
}
