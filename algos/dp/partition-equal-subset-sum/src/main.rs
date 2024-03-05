fn main() {
    println!("Hello, world!");
    // Output: true
    // Explanation: The array can be partitioned as [1, 5, 5] and [11].
    let nums = vec![1, 5, 11, 5];
    println!("{}", can_partition(nums));
    let nums = vec![1, 2, 3, 5];
    println!("{}", can_partition(nums));
}

pub fn can_partition(nums: Vec<i32>) -> bool {
    let mut hm = std::collections::HashMap::new();
    _can_partition(&nums, 0, 0, 0, &mut hm)
}

pub fn _can_partition(
    nums: &Vec<i32>,
    idx: usize,
    total_sum_so_far: i32,
    sum1: i32,
    hm: &mut std::collections::HashMap<(usize, i32), bool>,
) -> bool {
    if idx == nums.len() {
        //           22        1 + 5 + 5 = 11
        let sum2 = total_sum_so_far - sum1;
        return sum2 == sum1;
    }

    if let Some(ret) = hm.get(&(idx, sum1)) {
        return *ret;
    }

    // leave
    // println!("prev_sum={:?}", prev_sum);
    let choice1 = _can_partition(nums, idx + 1, total_sum_so_far + nums[idx], sum1, hm);
    if choice1 == true {
        hm.insert((idx, sum1), choice1);
        return true;
    }

    // pick
    // println!("prev_sum={:?}", prev_sum);
    let choice2 = _can_partition(
        nums,
        idx + 1,
        total_sum_so_far + nums[idx],
        sum1 + nums[idx],
        hm,
    );

    hm.insert((idx, sum1), choice2);
    choice2
}
