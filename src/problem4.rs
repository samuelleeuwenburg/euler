#![allow(dead_code)]

// A palindromic number reads the same both ways. The largest palindrome made from the product of
// two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.
fn problem() -> (u64, u64) {
    let mut factors = (0, 0);
    let mut highest_result = 0;

    for x in (100..1000).rev() {
        for y in (100..1000).rev() {
            let result = x * y;

            if result > highest_result && is_palindrome(result) {
                highest_result = result;
                factors = (x, y);
            }
        }
    }

    factors
}


fn is_palindrome(n: u64) -> bool {
    let string = n.to_string();
    let chars = string.chars().count();
    let pos_a = chars / 2;
    let pos_b = if chars % 2 == 0 { pos_a } else { pos_a + 1 };

    let a = string.get(0..pos_a).unwrap();
    let b: String = string.get(pos_b..chars).unwrap().chars().rev().collect();
    
    a == b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_works() {
        assert_eq!(problem(), (993, 913));
    }

    #[test]
    fn is_palindrome_works() {
        assert_eq!(is_palindrome(1221), true);
        assert_eq!(is_palindrome(1010101), true);
        assert_eq!(is_palindrome(1901091), true);
        assert_eq!(is_palindrome(121), true);

        assert_eq!(is_palindrome(1231), false);
        assert_eq!(is_palindrome(101010), false);
        assert_eq!(is_palindrome(10), false);
    }

}
