use std::collections::{HashSet, VecDeque};

fn main() {
    println!("Hello, world!");
    // Example 1:

    let nums = vec![2, 4, 12];
    let start = 2;
    let goal = 12;
    // Output: 2
    // Explanation: We can go from 2 → 14 → 12 with the following 2 operations.
    // - 2 + 12 = 14
    // - 14 - 2 = 12
    println!("{}", minimum_operations(nums, start, goal));

    //     Input: nums = [3,5,7], start = 0, goal = -4
    // Output: 2
    let nums = vec![3, 5, 7];
    let start = 0;
    let goal = -4;
    println!("{}", minimum_operations(nums, start, goal));
    //     Input: nums = [2,8,16], start = 0, goal = 1
    // Output: -1
    let nums = vec![2, 8, 16];
    let start = 0;
    let goal = 1;
    println!("{}", minimum_operations(nums, start, goal));
}

pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back(start);
    visited.insert(start);
    let mut lvl = 0;

    while !q.is_empty() {
        lvl += 1;
        let size = q.len();
        for _ in 0..size {
            let cur = q.pop_front().unwrap();
            if cur == goal {
                return lvl;
            }
            for neighbor in get_neighbors(cur, &nums) {
                if neighbor == goal {
                    return lvl;
                }
                if visited.insert(neighbor) {
                    if neighbor >= 0 && neighbor <= 1000 {
                        q.push_back(neighbor);
                    }
                }
            }
        }
    }

    -1
}

fn get_neighbors(cur: i32, nums: &Vec<i32>) -> Vec<i32> {
    let mut v = Vec::new();

    for i in 0..nums.len() {
        v.push(cur + nums[i]);
        v.push(cur - nums[i]);
        v.push(cur ^ nums[i]);
    }

    // println!("v={:?}", v);
    v
}
