fn main() {
    println!("Hello, world!");
    let nums = vec![1, 2, 3];
    println!("{:?}", permute(nums));
    let nums = vec![0, 1];
    println!("{:?}", permute(nums));
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut vec = vec![];
    _permute(&nums, &mut vec, 0, vec![]);
    vec
}

pub fn _permute(nums: &Vec<i32>, vec: &mut Vec<Vec<i32>>, idx: usize, mut v: Vec<i32>) {
    if idx == nums.len() {
        vec.push(v);
        return;
    }

    for i in 0..nums.len() {
        if !v.contains(&nums[i]) {
            v.push(nums[i]);
            _permute(nums, vec, idx + 1, v.clone());
            v.pop();
        }
    }
}
