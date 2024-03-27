fn main() {
    println!("Hello, world!");
}

pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    // get 2nd smallest pos and smallest pos, I have a test tomorrow linear algebra, I hope I did well
    // oh if the lowest is 1 we have aproblem
    // may be go from the two sides

    // you did well
    // for (int i = 0; i < n; i++) {
    //         // Check whether 1 is in the original array
    //         if (nums[i] == 1) {
    //             contains1 = true;
    //         }
    //         if (nums[i] <= 0 || nums[i] > n) {
    //             nums[i] = 1;
    //         }
    //     }
    let n = nums.len() as i32;
    let mut one_found = false;
    for num in nums.iter_mut() {
        if *num == 1 {
            one_found = true;
        }
        if *num <= 0 || *num > n {
            *num = 1;
        }
    }

    if !one_found {
        return 1;
    }

    for i in 0..n.try_into().unwrap_or(0) {
        let val = nums[i];
        if val == n {
            nums[0] = -nums[0].abs();
        } else {
            println!("val={:?}", val);
            nums[val as usize] = -nums[val as usize].abs();
        }
    }

    for i in 0..n.try_into().unwrap() {
        if nums[i].is_negative() {
            return i.try_into().unwrap();
        }
    }

    if nums[0] > 0 {
        return n;
    }

    n + 1
}
