fn main() {
    println!("Hello, world!");
}

pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    for i in (0..nums.len() - 2).rev() {
        if is_valid_triangle(nums[i], nums[i + 1], nums[i + 2]) {
            return nums[i] + nums[i + 1] + nums[i + 2];
        }
    }
    0
}

pub fn is_valid_triangle(x: i32, y: i32, z: i32) -> bool {
    if x + y > z && x + z > y && y + z > x {
        true
    } else {
        false
    }
}
