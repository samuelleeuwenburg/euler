#![allow(dead_code)]
use crate::prime::{sieve_of_eratosthenes};

fn problem(roof: u64) -> u64 {
    // @TODO replace for Sieve of Sundaram
    let primes = sieve_of_eratosthenes(roof);
    primes.into_iter().fold(0, |sum, p| sum + p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_test() {
        assert_eq!(problem(10), 17);
        assert_eq!(problem(2_000_000), 142913828922);
    }
}
