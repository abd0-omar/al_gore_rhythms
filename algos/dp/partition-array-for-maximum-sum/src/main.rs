fn main() {
    println!("Hello, world!");
    let arr = vec![1, 15, 7, 9, 2, 5, 10];
    let k = 3;
    // Output: 84
    // Explanation: arr becomes [15,15,15,9,10,10,10]
    println!("{}", partition(arr, k));
}

fn partition(arr: Vec<i32>, k: i32) -> i32 {
    _partition(&arr, k, 0)
}

fn _partition(arr: &Vec<i32>, k: i32, st_idx: usize) -> i32 {
    if st_idx == arr.len() {
        return 0;
    }

    let mut sum = 0;

    let mut max = 0;
    // let mut block_sum = 0;
    for end in st_idx..arr.len() {
        if k == 0 {
            break;
        }
        max = max.max(arr[end]);
        let len = end - st_idx + 1;
        // block_sum += max;
        // let total = block_sum + _partition(arr, k - 1, end + 1);
        let total = (len as i32 * max) + _partition(arr, k - 1, end + 1);
        sum = sum.max(total)
    }

    sum
}
