fn main() {
    let nums = vec![2, 2, 3, 4];
    let nums = vec![4, 2, 3, 4];
    println!("{:?}", triangle_number(nums));
}

pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return 0;
    }
    nums.sort();
    let mut count = 0;
    let n = nums.len();
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            let x = nums[i];
            let y = nums[j];
            let mut l = j as i32 + 1;
            let mut r = n as i32 - 1;
            let mut ans = -1;

            while l <= r {
                let mid = l + (r - l) / 2;

                if nums[mid as usize] < x + y {
                    ans = mid;
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }

            if ans != -1 {
                count += ans - j as i32;
            }
        }
    }
    count
}
