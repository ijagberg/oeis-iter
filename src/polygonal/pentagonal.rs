//! [OEIS A000326](https://oeis.org/A000326)
use super::PolygonalIterator;

pub type PentagonalIterator = PolygonalIterator<5>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hexagonal_iterator_test() {
        let first_47: Vec<_> = PentagonalIterator::new().take(47).collect();

        assert_eq!(
            first_47,
            vec![
                0, 1, 5, 12, 22, 35, 51, 70, 92, 117, 145, 176, 210, 247, 287, 330, 376, 425, 477,
                532, 590, 651, 715, 782, 852, 925, 1001, 1080, 1162, 1247, 1335, 1426, 1520, 1617,
                1717, 1820, 1926, 2035, 2147, 2262, 2380, 2501, 2625, 2752, 2882, 3015, 3151
            ]
        );
    }
}
