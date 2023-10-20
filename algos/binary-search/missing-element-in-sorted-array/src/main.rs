fn main() {
    let nums = vec![2, 3, 4, 7, 11];
    let k = 12; // 9
    println!("{}", missing_element(nums, k));
}

fn missing_element(nums: Vec<i32>, k: i32) -> i32 {
    let mut missing_count = 0;
    let mut current = nums[0];
    let mut found = nums[0];
    let mut i = 0;

    while missing_count < k {
        if i < nums.len() && nums[i] == current {
            i += 1;
        } else {
            found = current;
            missing_count += 1;
        }

        current += 1;
    }
    found
}
