pub struct TriangleIterator {
    current: u128,
}

impl TriangleIterator {
    pub fn new() -> Self {
        Self { current: 0 }
    }

    pub fn start_from(n: u128) -> Self {
        Self { current: n }
    }
}

impl Iterator for TriangleIterator {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.current;
        let out = nth_triangle_number(n);
        self.current += 1;

        Some(out)
    }
}

impl Default for TriangleIterator {
    fn default() -> Self {
        Self::new()
    }
}

fn nth_triangle_number(n: u128) -> u128 {
    (n * (n + 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_iterator_test() {
        let first_54: Vec<u128> = TriangleIterator::new().take(54).collect();

        assert_eq!(
            first_54,
            vec![
                0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190,
                210, 231, 253, 276, 300, 325, 351, 378, 406, 435, 465, 496, 528, 561, 595, 630,
                666, 703, 741, 780, 820, 861, 903, 946, 990, 1035, 1081, 1128, 1176, 1225, 1275,
                1326, 1378, 1431
            ]
        )
    }

    #[test]
    fn start_from_test() {
        let skipped_first_15: Vec<u128> = TriangleIterator::start_from(15).take(10).collect();
        assert_eq!(
            skipped_first_15,
            vec![120, 136, 153, 171, 190, 210, 231, 253, 276, 300]
        )
    }
}
