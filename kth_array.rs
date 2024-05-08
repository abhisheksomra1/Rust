fn quickselect(arr: &mut [i32], k: usize) -> i32 {
    let n = arr.len();
    if k > 0 && k <= n {
        let index = partition(arr, 0, n - 1);
        if index == k - 1 {
            return arr[index];
        } else if index > k - 1 {
            return quickselect(&mut arr[..index], k);
        } else {
            return quickselect(&mut arr[index + 1..], k - index - 1);
        }
    }
    panic!("Invalid value of k");
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;
    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

fn main() {
    let mut arr = [7, 10, 4, 3, 20, 15];
    let k = 3;
    let kth_smallest = quickselect(&mut arr, k);
    println!("The {}th smallest element is: {}", k, kth_smallest);
}
