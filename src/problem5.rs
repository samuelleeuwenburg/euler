#![allow(dead_code)]
use crate::problem3::{Primes, get_prime_factorization};

// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.  
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
fn problem(num: usize) -> u64 {
    let mut primes = Primes::new();
    let mut final_factors: Vec<u64> = vec![];

    for n in 2..(num + 1) {
        if primes.is_prime(n as u64) {
            final_factors.push(n as u64);
            continue;
        }

        // merge factors without removing duplicates
        // @TODO: find a way to write this elegantly
        let mut factors = final_factors.clone();
        for p in get_prime_factorization(n as u64) {
            let result = factors
                .iter()
                .enumerate()
                .find_map(|(i, &x)| if x == p { Some(i) } else { None });

            match result {
                Some(i) => { factors.remove(i); }
                None => final_factors.push(p as u64)
            }
        }
    }

    final_factors.into_iter().fold(1, |acc, p| acc * p)
}

fn brute_force(num: u64) -> f64 {
    let mut x = 1f64;

    'check: loop {
        for y in (2..num + 1).rev() {
            if (x / y as f64).fract() != 0.0 {
                x = x + 1.0;
                continue 'check;
            }
        }

        return x
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_test() {
        assert_eq!(problem(10), 2520);
        assert_eq!(problem(20), 232_792_560);
        assert_eq!(problem(42), 219_060_189_739_591_200);
    }

    #[test]
    #[ignore]
    fn brute_force_test() {
        assert_eq!(brute_force(3), 6.0);
        assert_eq!(brute_force(4), 12.0);
        assert_eq!(brute_force(5), 60.0);
        assert_eq!(brute_force(6), 60.0);
        assert_eq!(brute_force(7), 420.0);
        assert_eq!(brute_force(8), 840.0);
        assert_eq!(brute_force(9), 2520.0);
        assert_eq!(brute_force(10), 2520.0);
        assert_eq!(brute_force(20), 232792560.0);
    }
}
