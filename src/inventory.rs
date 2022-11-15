//! [OEIS A342585](https://oeis.org/A342585)

use std::collections::{hash_map::Entry, HashMap};

pub struct InventoryIterator {
    current: u128,
    counts: HashMap<u128, u128>,
}

impl InventoryIterator {
    fn _new(current: u128, counts: HashMap<u128, u128>) -> Self {
        Self { current, counts }
    }

    pub fn new() -> Self {
        Self::_new(0, HashMap::new())
    }

    fn increment_count_for(&mut self, number: u128) {
        match self.counts.entry(number) {
            Entry::Occupied(mut occupied) => {
                let count = occupied.get_mut();
                *count += 1;
            }
            Entry::Vacant(vacant) => {
                vacant.insert(1);
            }
        }
    }

    fn get_count(&self, number: u128) -> u128 {
        *self.counts.get(&number).unwrap_or(&0)
    }
}

impl Default for InventoryIterator {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for InventoryIterator {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        let count_of_current = self.get_count(current);
        self.increment_count_for(count_of_current);
        if count_of_current == 0 {
            // take inventory
            self.current = 0;
        } else {
            self.current += 1;
        }

        Some(count_of_current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_iterator_test() {
        let first_85: Vec<u128> = InventoryIterator::new().take(85).collect();
        assert_eq!(
            first_85,
            vec![
                0, 1, 1, 0, 2, 2, 2, 0, 3, 2, 4, 1, 1, 0, 4, 4, 4, 1, 4, 0, 5, 5, 4, 1, 6, 2, 1, 0,
                6, 7, 5, 1, 6, 3, 3, 1, 0, 7, 9, 5, 3, 6, 4, 4, 2, 0, 8, 9, 6, 4, 9, 4, 5, 2, 1, 3,
                0, 9, 10, 7, 5, 10, 6, 6, 3, 1, 4, 2, 0, 10, 11, 8, 6, 11, 6, 9, 3, 2, 5, 3, 2, 0,
                11, 11, 10
            ]
        );
    }
}
