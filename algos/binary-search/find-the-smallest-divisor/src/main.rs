fn main() {
    let nums = vec![1, 2, 5, 9];
    let threshold = 6;
    println!("{}", smallest_divisor(nums, threshold));
    // Output: 5
    // Explanation: We can get a sum to 17 (1+2+5+9) if the divisor is 1.
    // If the divisor is 4 we can get a sum of 7 (1+1+2+3) and if the divisor is 5 the sum will be 5 (1+1+1+2).

    let nums = vec![44, 22, 33, 11, 1];
    let threshold = 5;
    // Output: 44
}

pub fn possible_without_ceil(mid: i32, nums: &Vec<i32>, threshold: i32) -> bool {
    let mut sum: i32 = 0;
    for num in nums {
        // num / mid
        sum += (num + mid - 1) / mid;
    }
    sum <= threshold
}

pub fn possible(mid: i32, nums: &Vec<i32>, threshold: i32) -> bool {
    println!("DEBUGPRINT[2]: main.rs:14: mid={:#?}", mid);
    let mut sum: i32 = 0;
    for num in nums {
        let mult: f32 = (*num as f32 / mid as f32).ceil();
        sum += mult as i32;
        println!("DEBUGPRINT[1]: main.rs:19: num/mid={:#?}", mult);
    }
    println!("DEBUGPRINT[1]: main.rs:19: sum={:#?}", sum);
    sum <= threshold
}

pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    //sum <= threshold
    let mut l = 1;
    let mut r = *nums.iter().max().unwrap();
    let mut ans = -1;

    while l <= r {
        let mid = l + (r - l) / 2;

        match possible_without_ceil(mid, &nums, threshold) {
            true => {
                r = mid - 1;
                ans = mid;
            }
            false => l = mid + 1,
        }
    }
    ans
}
