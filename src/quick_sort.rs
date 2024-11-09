// quick_sort.rs

pub fn sort(arr: &mut Vec<i32>) {
    quick_sort(arr, 0, arr.len() as isize - 1);
}

fn quick_sort(arr: &mut Vec<i32>, low: isize, high: isize) {
    if low < high {
        let pivot_index = partition(arr, low, high);
        quick_sort(arr, low, pivot_index - 1);
        quick_sort(arr, pivot_index + 1, high);
    }
}

fn partition(arr: &mut Vec<i32>, low: isize, high: isize) -> isize {
    let pivot = arr[high as usize];
    let mut i = low - 1;
    for j in low..high {
        if arr[j as usize] <= pivot {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, high as usize);
    i + 1
}
