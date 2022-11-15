//! [OEIS A000045](https://oeis.org/A000045)

/// Iterator that visits the Fibonacci numbers (0, 1, 1, 2...)
///
/// ## Example
/// ```rust
/// # use oeis_iter::fibonacci::*;
/// let first_5: Vec<i128> = FibonacciIterator::new().take(5).collect();
/// assert_eq!(first_5, vec![0, 1, 1, 2, 3]);
/// ```
pub struct FibonacciIterator {
    a: i128,
    b: i128,
}

impl FibonacciIterator {
    fn _new(a: i128, b: i128) -> Self {
        Self { a, b }
    }

    pub fn new() -> Self {
        Self::_new(-1, 1)
    }
}

impl Iterator for FibonacciIterator {
    type Item = i128;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_iterator_test() {
        let first_41: Vec<_> = FibonacciIterator::new().into_iter().take(41).collect();

        assert_eq!(
            first_41,
            vec![
                0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
                6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040,
                1346269, 2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169,
                63245986, 102334155
            ]
        );
    }
}
