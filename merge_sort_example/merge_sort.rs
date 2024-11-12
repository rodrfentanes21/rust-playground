use std::time::Instant;

fn merge_sort<T: Ord + Clone + std::marker::Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    
    let mid = len / 2;
    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..]);
    merge(arr, mid);
}

fn merge<T: Ord + Clone + std::marker::Copy>(arr: &mut [T], mid: usize) {
    let mut temp = Vec::with_capacity(arr.len());
    let (left, right) = arr.split_at_mut(mid);

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            temp.push(left[i].clone());
            i += 1;
        } else {
            temp.push(right[j].clone());
            j += 1;
        }
    }

    temp.extend_from_slice(&left[i..]);
    temp.extend_from_slice(&right[j..]);

    arr.copy_from_slice(&temp);
}

fn main() {
    let mut arr = [5, 3, 8, 1, 2, 7, 4, 6, 9, 10];
    let start = Instant::now();
    merge_sort(&mut arr);
    let duration = start.elapsed();

    println!("{:?}", arr);
    println!("Time taken to sort: {:?}", duration);
}
