#![allow(dead_code)]
use crate::prime::find_primes_up_to;

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is
// 13.
//
// What is the 10 001st prime number?
fn problem(num: usize) -> u64 {
    let primes = find_primes_up_to(10_000);
    primes.get(num - 1).unwrap().to_owned()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_test() {
        assert_eq!(problem(6), 13);
        assert_eq!(problem(1001), 7927);
    }
}
