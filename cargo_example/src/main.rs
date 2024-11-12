use rand::seq::SliceRandom;
use std::time::Instant;

fn bogo_sort<T: Ord>(arr: &mut [T]) {
    while !is_sorted(arr) {
        arr.shuffle(&mut rand::thread_rng());
    }
}

fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

fn main() {
    let mut arr = [5, 3, 8, 1, 2, 7, 4, 6, 9, 10];
    arr.shuffle(&mut rand::thread_rng());

    let start = Instant::now();
    bogo_sort(&mut arr);
    let duration = start.elapsed();

    println!("Sorted array: {:?}", arr);
    println!("Time taken to sort: {:?}", duration);
}