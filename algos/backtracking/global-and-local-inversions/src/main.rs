fn main() {
    println!("Hello, world!");
    //     Input: nums = [1,0,2]
    // Output: true
    // Explanation: There is 1 global inversion and 1 local inversion.
    // Example 2:
    //
    // Input: nums = [1,2,0]
    // Output: false
    // Explanation: There are 2 global inversions and 1 local inversion.
    let nums = vec![10, 30, 2, 7, 27, 5, 37, 20];
    // let mut tmp =
    // println!("{:?}", _is_ideal_permutation(&nums, 0, 0));
    let nums=vec![7, 3, 6, 5, 1, 8, 4, 2];
    let nums=vec![2, 4, 7, 6, 5, 8 , 3 , 1];
// let nums=vec![7, 3, 6, 1, 8, 4, 5, 2];
    merge_sort(nums);
}

// handles duplicates too
pub fn merge_sort(nums: Vec<i32>) -> bool {
    let mut tmp = nums.clone();
    let mut global_inversions = 0;
    let mut local_inversions = 0;
    for i in 0..nums.len() - 1 {
        if nums[i] > nums[i + 1] {
            local_inversions += 1;
        }
    }
    _merge_sort(
        &mut nums.clone(),
        0,
        nums.len() - 1,
        &mut tmp,
        &mut global_inversions,
    );
    println!("global_inversions={:#?}", global_inversions);
    println!("sorted={:#?}", tmp);
    global_inversions == local_inversions
}

pub fn _merge_sort(
    nums: &mut Vec<i32>,
    st: usize,
    end: usize,
    tmp: &mut Vec<i32>,
    global_inversions: &mut i32,
) {
    if st == end {
        return;
    }

    let mid = st + (end - st) / 2;

    _merge_sort(nums, st, mid, tmp, global_inversions);
    _merge_sort(nums, mid + 1, end, tmp, global_inversions);

    join_sorted_arrays(nums, st, mid, end, tmp, global_inversions);

    // Copy the sorted values back to the original vector
    // for i in st..=end {
    //     nums[i] = tmp[i];
    // }
}

pub fn join_sorted_arrays(
    nums: &mut Vec<i32>,
    st: usize,
    mid: usize,
    end: usize,
    tmp: &mut Vec<i32>,
    global_inversions: &mut i32,
) {
    println!("end={:#?}", end);
    let mut i = st;
    let mut j = mid + 1;
    let mut k = st;

    while k <= end {
        if i > mid {
            tmp[k] = nums[j];
            j += 1;
        } else if j > end {
            tmp[k] = nums[i];
            i += 1;
        } else if nums[j] > nums[i] {
            tmp[k] = nums[i];
            i += 1;
        } else {
            tmp[k] = nums[j];
            j += 1;
            *global_inversions += mid as i32 - i as i32 + 1;
        }

        k += 1;
    }

    println!("{:?}", &tmp[st..=end]);
    nums[st..=end].copy_from_slice(&tmp[st..=end]);
}
