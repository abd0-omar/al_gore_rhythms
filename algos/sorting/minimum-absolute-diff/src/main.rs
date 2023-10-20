use std::cmp::min;
fn main() {
    let arr = vec![1, 3, 6, 10, 15];
    println!("{:?}", minimum_abs_difference(arr));
}

pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut arr = arr;
    let mut rez: Vec<Vec<i32>> = Vec::new();
    let mut lowest: i32 = std::i32::MAX;

    arr.sort();

    for i in 1..arr.len() {
        lowest = min(arr[i] - arr[i - 1], lowest);
    }

    for i in 1..arr.len() {
        if arr[i] - arr[i - 1] == lowest {
            let tmp = vec![arr[i - 1], arr[i]];
            rez.push(tmp);
        }
    }

    rez
}
