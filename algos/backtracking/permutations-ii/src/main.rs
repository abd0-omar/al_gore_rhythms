use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let nums = vec![1, 1, 2];
    permute_unique(nums);
    let nums = vec![1, 2, 3];
    permute_unique(nums);
}

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut freq = HashMap::new();
    for &num in nums.iter() {
        *freq.entry(num).or_insert(0) += 1;
    }
    // println!("freq={:?}", freq);
    let mut rezult = Vec::new();
    _permute_unique(&nums, freq, &mut rezult, vec![], 0);
    println!("rezult={:?}", rezult);
    rezult
}

pub fn _permute_unique(
    nums: &Vec<i32>,
    mut freq: HashMap<i32, i32>,
    rezult: &mut Vec<Vec<i32>>,
    mut curr_vec: Vec<i32>,
    idx: usize,
) {
    if idx == nums.len() {
        rezult.push(curr_vec);
        return;
    }

    // for rust ownership rules
    // to satisfy the compiler
    let cloned_freq = freq.clone();

    for (key, val) in cloned_freq.iter() {
        if *val > 0 {
            // inside the loop, use the original mutable freq
            *freq.get_mut(key).unwrap() -= 1;
            // curr_vec[idx] = *key;
            curr_vec.push(*key);
            // pass the cloned freq hashmap into the recursive call
            _permute_unique(nums, freq.clone(), rezult, curr_vec.clone(), idx + 1);
            curr_vec.pop();
            // curr_vec[idx] = -1;
            *freq.get_mut(key).unwrap() += 1;
        }
    }
}
