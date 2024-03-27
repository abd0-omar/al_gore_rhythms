fn main() {
    println!("Hello, world!");
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let first = nums.partition_point(|&x| x < target);
    if first == nums.len() || nums[first] != target {
        return vec![-1, -1];
    }
    // reducing the right side search space
    let last = nums[first..nums.len()].partition_point(|&x| x <= target) - 1;
    // readiing the first index to the right side
    vec![first as i32, (last + first) as i32]
}
