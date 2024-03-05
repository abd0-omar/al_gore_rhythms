fn main() {
    println!("Hello, world!");
    let nums = vec![1, 2, 3];
    permute(nums);
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut rezult = Vec::new();
    _permute(&nums, nums.clone(), &mut rezult, 0);
    println!("rezult={:?}", rezult);
    rezult
}

pub fn _permute(nums: &Vec<i32>, mut curr_vec: Vec<i32>, rezult: &mut Vec<Vec<i32>>, idx: usize) {
    if idx == nums.len() {
        rezult.push(curr_vec.clone());
        return;
    }

    for i in idx..nums.len() {
        curr_vec.swap(i, idx);
        _permute(nums, curr_vec.clone(), rezult, idx + 1);
        curr_vec.swap(i, idx);
    }
}
