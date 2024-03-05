fn main() {
    println!("Hello, world!");
}

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut memory = [[None; 2500]; 2500];
    _rob(&nums, 0, None, &mut memory)
}

fn _rob(
    nums: &Vec<i32>,
    idx: usize,
    prev: Option<usize>,
    memory: &mut [[Option<i32>; 2500]; 2500],
) -> i32 {
    // prev none 3ady bra7tak a5taro aw la2
    // law el prev m4 none, f lazm yb2a b3eed b wa7ed ela el 22l
    if idx == nums.len() {
        return 0;
    }

    if let Some(p) = prev {
        if let Some(val) = memory[idx][p] {
            return val;
        }
    }

    // leave
    let choice1 = _rob(nums, idx + 1, prev, memory);
    let mut choice2 = 0;
    if let Some(p) = prev {
        if idx - p >= 2 {
            // pick if there is prev and two places apart
            choice2 = nums[idx] + _rob(nums, idx + 1, Some(idx), memory);
        }
    } else {
        // pick there is no prev
        choice2 = nums[idx] + _rob(nums, idx + 1, Some(idx), memory);
    }
    memory[idx][prev.unwrap()] = Some(choice1.max(choice2));
    choice1.max(choice2)
}
