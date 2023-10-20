fn main() {
    println!("Hello, world!");
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut low = 0;
    let mut high = nums.len() as i32 - 1;
    let mut first = -1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if nums[mid as usize] >= target {
            first = mid;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    if first == -1 || nums[first as usize] != target {
        return vec![-1, -1];
    }

    let mut last = -1;
    low = 0;
    high = nums.len() as i32 - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if nums[mid as usize] <= target {
            last = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    vec![first, last]
}
