use oeis_iter::inventory::InventoryIterator;
use std::env::args;

fn main() {
    let count: usize = args().nth(1).unwrap().parse().unwrap();
    let sequence: Vec<u128> = InventoryIterator::new().take(count).collect();

    for num in sequence {
        println!("{num}");
    }
}
