#![allow(dead_code)]

// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By
// starting with 1 and 2, the first 10 terms will be:
// 
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
// 
// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find
// the sum of the even-valued terms.
fn problem() -> usize {
    fibonacci(4_000_000)
        .into_iter()
        .filter(|x| x % 2 == 0)
        .fold(0, |acc, x| acc + x)
}

fn fibonacci(max: usize) -> Vec<usize> {
    fn rec(mut acc: Vec<usize>, max: usize, x: usize, y: usize) -> Vec<usize> {
        if x > max {
            acc
        } else {
            acc.push(x);
            rec(acc, max, x + y, x)
        }
    }

    rec(vec![], max, 1, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_test() {
        assert_eq!(fibonacci(21), [1, 2, 3, 5, 8, 13, 21]);
    }

    #[test]
    fn problem_test() {
        assert_eq!(problem(), 4_613_732);
    }
}
