
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        
        if arr[mid] == target {
            // Check if the element at mid is the first occurrence
            if mid == 0 || arr[mid - 1] != target {
                return Some(mid);
            } else {
                // Continue searching towards the left for the first occurrence
                right = mid - 1;
            }
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

fn main() {
    let arr = vec![1, 2, 3, 3, 4, 5, 5, 6, 7, 8];
        let target = 4;
    
    match first_occurrence(&arr, target) {
        Some(index) => println!("First occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
