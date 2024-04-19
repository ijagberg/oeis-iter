pub mod heptagonal;
pub mod hexagonal;
pub mod pentagonal;
pub mod square;
pub mod triangle;

pub struct PolygonalIterator<const SIDES: u128> {
    current: u128,
}

impl<const SIDES: u128> PolygonalIterator<SIDES> {
    fn _new(n: u128) -> Self {
        if SIDES < 3 {
            panic!("PolygonalIterator with fewer than three sides");
        }
        Self { current: n }
    }

    /// Create a new `PolygonalIterator`
    ///
    /// ## Panics
    /// If `SIDES` is less than 3.
    pub fn new() -> Self {
        Self::_new(0)
    }

    /// Start this iterator at the `n`th element.
    ///
    /// ## Example
    /// ```rust
    /// # use oeis_iter::polygonal::*;
    /// let fourth_fifth: Vec<u128> = PolygonalIterator::<3>::start_from(4).take(2).collect();
    /// assert_eq!(fourth_fifth, vec![10, 15]);
    /// ```
    pub fn start_from(n: u128) -> Self {
        Self::_new(n)
    }

    fn polygonal_nth(&self, n: u128) -> u128 {
        let s_minus_2 = SIDES - 2;
        let n_times_n_minus_1 = n * (n.saturating_sub(1));
        (s_minus_2 * (n_times_n_minus_1 / 2)) + n
    }
}

impl<const SIDES: u128> Iterator for PolygonalIterator<SIDES> {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.polygonal_nth(self.current);
        self.current += 1;
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polygonal_test() {
        let triangle: PolygonalIterator<3> = PolygonalIterator::new();

        assert_eq!(triangle.take(5).collect::<Vec<_>>(), vec![0, 1, 3, 6, 10]);

        let square: PolygonalIterator<4> = PolygonalIterator::new();

        assert_eq!(square.take(5).collect::<Vec<_>>(), vec![0, 1, 4, 9, 16]);

        let pentagonal: PolygonalIterator<5> = PolygonalIterator::new();

        assert_eq!(
            pentagonal.take(5).collect::<Vec<_>>(),
            vec![0, 1, 5, 12, 22]
        );
    }

    #[test]
    #[should_panic]
    fn new_panics_test() {
        let _iter: PolygonalIterator<2> = PolygonalIterator::new();
    }
}
