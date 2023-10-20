fn main() {
    let intervals = vec![vec![1, 4], vec![2, 3], vec![3, 4]];
    let output = vec![-1, 2, -1];
    println!("{:?}", find_right_interval(intervals));
    let intervals = vec![vec![3, 4], vec![2, 3], vec![1, 2]];
    let output = vec![-1, 0, 1];
    println!("{:?}", find_right_interval(intervals));
    // let vec = vec![2, 4, 6, 8, 10, 15, 20, 21, 30];
    // find_first_bigger_or_equal(vec);
}

//first time baby LETS" GOOOOO!
pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ascen: bool;
    let n = intervals.len();

    if intervals[0][0] < intervals[n - 1][0] {
        ascen = true;
    } else {
        ascen = false;
    }

    let mut ans_vec: Vec<i32> = Vec::with_capacity(n);

    if ascen {
        for i in 0..n {
            let mut l = 0;
            let mut r = n as i32 - 1;
            let mut ans = -1;
            let target = intervals[i][1];

            while l <= r {
                let mid = l + (r - l) / 2;
                // if mid == i {
                //     //continue;
                // }
                if intervals[mid as usize][0] >= target {
                    r = mid - 1;
                    ans = mid;
                    println!("DEBUGPRINT[2]: main.rs:27: ans={:#?}", ans);
                } else if intervals[mid as usize][0] < target {
                    l = mid + 1;
                }
            }
            ans_vec.push(ans);
        }
    } else {
        for i in 0..n {
            let mut l = 0;
            let mut r = n as i32 - 1;
            let mut ans = -1;
            let target = intervals[i][1];

            while l <= r {
                let mid = l + (r - l) / 2;
                // if mid == i {
                //     //continue;
                // }
                if intervals[mid as usize][0] >= target {
                    l = mid + 1;
                    ans = mid;
                    println!("DEBUGPRINT[2]: main.rs:27: ans={:#?}", ans);
                } else if intervals[mid as usize][0] < target {
                    r = mid - 1;
                }
            }
            ans_vec.push(ans);
        }
    }

    println!("DEBUGPRINT[3]: main.rs:43: ans_vec={:#?}", ans_vec);

    ans_vec
}

fn find_first_bigger_or_equal(nums: Vec<i32>) -> () {
    let n = nums.len();
    for i in 0..n {
        let mut l = 0;
        let mut r = n as i32 - 1;
        let target = nums[i];
        let mut ans = -1;

        while l <= r {
            let mid = l + (r - l) / 2;
            if target <= nums[mid as usize] {
                ans = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        println!("{}", ans);
    }
}
