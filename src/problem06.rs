#![allow(dead_code)]
// The sum of the squares of the first ten natural numbers is,
// 1^2 + 2^2 + ... + 10^2 = 385
// 
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)^2 = 552 = 3025
// 
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
// 
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
fn problem(num: u64) -> u64 {
    let sum_squares = (1..num + 1).fold(0, |acc, x| acc + x.pow(2));
    let square_sum = (1..num + 1).fold(0, |acc, x| acc + x).pow(2);

    square_sum - sum_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_test() {
        assert_eq!(problem(10), 2640);
        assert_eq!(problem(100), 25164150);
    }
}
