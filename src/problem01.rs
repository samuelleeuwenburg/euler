#![allow(dead_code)]

// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
//
// Find the sum of all the multiples of 3 or 5 below 1000.
pub fn problem() -> usize {
    (1..1000).fold(0, |acc, x| {
        if x % 3 == 0 || x % 5 == 0 {
            acc + x
        } else {
            acc
        }
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_result() {
        assert_eq!(problem(), 233168);
    }
}

