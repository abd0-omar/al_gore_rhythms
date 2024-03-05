fn main() {
    println!("Hello, world!");
    //     Example 1:
    //
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    // let nums = vec![9, 2, 5, 3, 7];
    // Output: 4
    // Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
    println!("{}", length_of_lis(nums.clone()));
    println!("{}", lis(nums));
}

pub fn lis(nums: Vec<i32>) -> i32 {
    let mut memory = [None; 1000];

    let mut ret = 0;

    for i in 0..nums.len() {
        let start = _lis(&nums, i, &mut memory);
        println!("start={:#?}", start);
        ret = ret.max(start);
    }

    ret
}

fn _lis(nums: &Vec<i32>, idx: usize, memory: &mut [Option<i32>; 1000]) -> i32 {
    if idx == nums.len() {
        return 0;
    }

    if let Some(val) = memory[idx] {
        return val;
    }

    let mut ret = 0;

    for j in idx + 1..nums.len() {
        if nums[j] > nums[idx] {
            ret = ret.max(_lis(nums, j, memory));
        }
    }

    ret += 1;
    memory[idx] = Some(ret);
    ret
}

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut memory = [[None; 1000]; 1000];
    _length_of_list(&nums, 0, None, &mut memory)
}

fn _length_of_list(
    nums: &Vec<i32>,
    idx: usize,
    prev_idx: Option<usize>,
    memory: &mut [[Option<i32>; 1000]; 1000],
) -> i32 {
    if idx == nums.len() {
        return 0;
    }

    // memomization
    if let Some(prev) = prev_idx {
        if let Some(ret) = memory[idx][prev] {
            return ret;
        }
    }

    let choice1 = _length_of_list(nums, idx + 1, prev_idx, memory);

    let mut choice2 = 0;

    if let Some(prev) = prev_idx {
        // if not first time then check if it is valid
        if nums[idx] > nums[prev] {
            choice2 = 1 + _length_of_list(nums, idx + 1, Some(idx), memory);
            memory[idx][prev_idx.unwrap()] = Some(choice1.max(choice2));
        }
    } else {
        // first time when prev_idx is None
        choice2 = 1 + _length_of_list(nums, idx + 1, Some(idx), memory);
    }

    choice1.max(choice2)
}
