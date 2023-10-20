fn main() {
    let input1 = vec![2, 6, 4, 8, 10, 9, 15];
    let input2 = vec![2, 6, 4, 15, 18, 17, 16, 30];
    let input3 = vec![2, 1];
    let input4 = vec![5, 4, 3, 2, 1];
    let output = find_unsorted_subarray(input4);
    println!("DEBUGPRINT[3]: main.rs:3: output={:#?}", output);
}

pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut nums_sorted = nums.clone();
    nums_sorted.sort();

    let mut result_len = 0;

    let mut st: i32 = -1;
    let mut end: i32 = -1;

    for i in 0..nums.len() {
        if nums[i] != nums_sorted[i] {
            if st == -1 {
                st = i as i32;
            } else {
                end = i as i32;
            }
        }
    }

    if st == -1 {
        return 0;
    }

    end - st + 1

    /*
    *     let mut nums = nums;
    let mut nums_sorted = nums.clone();
    nums_sorted.sort();
    let n = nums.len();

    let mut result_len = 0;


    let mut st = 0;
    let mut flag = false;
    for end in 0..n {
        if nums[end] == nums_sorted[end] {
            println!(
                "DEBUGPRINT[4]: main.rs:16: nums={:#?}, {:#?}",
                nums[st], nums[end]
            );
            result_len = (end - st) as i32 - 1;
            println!("DEBUGPRINT[5]: main.rs:22: result_len={:#?}", result_len);
            // st = end;
        } else {
            if !flag {
                flag = true;
            }
        }
    }


    let mut st = 0;
    let mut end = 0;
    let mut flag = false;
    for i in 0..n {
        if nums[end] != nums_sorted[end] {
            println!(
                "DEBUGPRINT[4]: main.rs:16: nums={:#?}, {:#?}",
                nums[st], nums[end]
            );
            end = i;
            result_len = (end - st) as i32 - 1;
            println!("DEBUGPRINT[5]: main.rs:22: result_len={:#?}", result_len);
            // st = end;
        } else {
            if !flag {
                flag = true;
            }
        }
    }

    k

    if !flag {
        println!("OK");
        return 0;
    }
    if flag && result_len == 0 {
        result_len = n as i32;
    }

    let mut reversed = nums.clone();
    reversed.reverse();
    if reversed == nums_sorted {
        return n as i32;
    }

    // let mut result_len = 0;
    //
    // for st in 0..n {
    //     for end in st..n {
    //         if nums[end] == nums_sorted[end] {
    //             println!(
    //                 "DEBUGPRINT[4]: main.rs:16: nums={:#?}, {:#?}",
    //                 nums[st], nums[end]
    //             );
    //             result_len = std::cmp::max(result_len, (end - st) as i32 - 1);
    //             println!("DEBUGPRINT[5]: main.rs:22: result_len={:#?}", result_len);
    //         }
    //     }
    // }

    // let mut result_len = n;
    //
    // let mut end = n;
    // let mut st = 0;
    // for _ in 0..n / 2 {
    //     let mut sorted_slice = &mut nums[st..end];
    //
    //     println!(
    //         "DEBUGPRINT[6]: main.rs:47: sorted_slice={:#?}",
    //         sorted_slice
    //     );
    //
    //     if sorted_slice == &nums_sorted[st..end] {
    //         println!("OK");
    //     }
    //
    //     st += 1;
    //     end -= 1;
    // }
    //
    result_len as i32
    * */
}
