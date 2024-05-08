fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        // Even number of elements
        let mid = len / 2;
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        // Odd number of elements
        arr[len / 2] as f64
    }
}

fn main() {
    let arr1 = vec![1, 2, 3, 4, 5];
    let arr2 = vec![1, 2, 3, 4, 5, 6];

    println!("Median of arr1: {}", find_median(&arr1));
    println!("Median of arr2: {}", find_median(&arr2));
}
