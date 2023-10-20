fn main() {
    let input = vec![3, -1, 0, 2];
    let input2 = vec![4, 2, 3];
    let input3 = vec![2, -3, -1, 4, -5];
    let output = largest_sum_after_k_negations(input3, 2);
}

pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut k = k;
    let mut sum = 0;
    let mut min = i32::MAX;

    for i in 0..nums.len() {
        if nums[i] < 0 && k > 0 {
            k -= 1;
            nums[i] *= -1;
        }

        sum += nums[i];
        min = std::cmp::min(min, nums[i]);
    }

    if k % 2 == 1 {
        //remove the added positive number
        sum -= min * 2;
    }
    sum
}
