fn main() {
    let nums = vec![1, 1, 3, 3, 5, 5, 5];
    let nums2 = vec![1, 1, 3, 3, 5, 5, 5, 7, 7, 7];
    let nums3 = vec![5, 1, 3];
    let nums4 = vec![1, 1, 1];
    let nums5 = vec![1, 1, 2, 2, 3];
    println!("{}", reduction_operations(nums5));
}

//solved it in exactly 17 mins didn't take any time, and I was taking my time implementing and
//thinking
pub fn reduction_operations(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut next_min_count = 0;

    let n = nums.len();
    let mut sum = 0;

    for i in 1..n {
        if nums[i] > nums[i - 1] {
            next_min_count += 1;
        }

        sum += next_min_count;
    }

    sum
}
