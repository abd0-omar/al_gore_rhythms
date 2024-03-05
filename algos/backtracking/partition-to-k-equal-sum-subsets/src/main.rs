fn main() {
    println!("Hello, world!");

    // let nums = vec![1, 2, 3, 4];
    // let k = 3;
    // // Output: false
    // println!("{}", can_partition_k_subsets(nums, k));
    // let nums = vec![4, 3, 2, 3, 5, 2, 1];
    // let k = 4;
    // println!("{}", can_partition_k_subsets(nums, k));

    // let nums = vec![1, 1, 1, 1, 2, 2, 2, 2];
    // let k = 4;
    // println!("{}", can_partition_k_subsets(nums, k));

    // let nums = vec![1, 2, 3, 5];
    // let k = 2;
    // println!("{}", can_partition_k_subsets(nums, k));
}

pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    let n = nums.len();
    let mut visited = vec![false; n];
    let sum_per_part = nums.iter().sum::<i32>() / k;
    println!("sum_per_part={:?}", sum_per_part);
    let mut for_printing = vec![vec![]; k as usize];
    let rez = _can_partition_k_subsets(
        &nums,
        k as usize,
        &mut visited,
        0,
        0,
        0,
        sum_per_part as usize,
        &mut for_printing,
    );

    if for_printing.iter().flatten().count() < nums.len() {
        false
    } else {
        rez
    }
}

pub fn _can_partition_k_subsets(
    nums: &Vec<i32>,
    k: usize,
    visited: &mut Vec<bool>,
    part_time: usize,
    idx: usize,
    cur_sum: usize,
    sum_per_part: usize,
    for_printing: &mut Vec<Vec<i32>>,
) -> bool {
    println!("for_printing={:?}", for_printing);
    // base cases
    if part_time == k {
        return true;
    }

    if cur_sum == sum_per_part {
        return _can_partition_k_subsets(
            nums,
            k,
            visited,
            part_time + 1,
            0,
            0,
            sum_per_part,
            for_printing,
        );
    }

    if idx == nums.len() {
        return false;
    }
    // end of base cases

    if !visited[idx] && cur_sum + nums[idx] as usize <= sum_per_part {
        // update
        visited[idx] = true;
        for_printing[part_time].push(nums[idx]);
        // pick
        if _can_partition_k_subsets(
            nums,
            k,
            visited,
            part_time,
            idx + 1,
            cur_sum + nums[idx] as usize,
            sum_per_part,
            for_printing,
        ) {
            return true;
        }
        // undo
        for_printing[part_time].pop();
        visited[idx] = false;
    }
    // leave
    if _can_partition_k_subsets(
        nums,
        k,
        visited,
        part_time,
        idx + 1,
        cur_sum,
        sum_per_part,
        for_printing,
    ) {
        return true;
    }

    false
}
