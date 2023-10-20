pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut arr = Vec::new();
    arr = arr.clone();
    arr.sort();
    let mut rez: Vec<Vec<i32>> = Vec::new();

    let mut lowest: i32 = std::i32::MAX;
    for i in 1..arr.len() {
        lowest = std::cmp::min(arr[i], arr[i - 1]);
    }
    for i in 1..arr.len() {
        if arr[i] - arr[i - 1] == lowest {
            let tmp = vec![arr[i], arr[i - 1]];
            rez.push(tmp);
        }
    }
    rez
}

#[test]
fn ex1() {
    unimplemented!();
}
// Example 1:
//
// Input: arr = [4,2,1,3]
// Output: [[1,2],[2,3],[3,4]]
// Explanation: The minimum absolute difference is 1. List all pairs with difference equal to 1 in ascending order.
// Example 2:
//
// Input: arr = [1,3,6,10,15]
// Output: [[1,3]]
// Example 3:
//
// Input: arr = [3,8,-10,23,19,-4,-14,27]
// Output: [[-14,-10],[19,23],[23,27]]
