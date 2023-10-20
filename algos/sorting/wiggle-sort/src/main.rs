fn main() {
    println!("Hello, world!");
    let mut input = vec![3, 5, 2, 1, 6, 4];
    wiggle_sort(&mut input);
}

fn wiggle_sort(nums: &mut Vec<i32>) {
    let mut nums = nums;
    nums.sort();
    let mut rez: Vec<i32> = Vec::with_capacity(nums.len());

    for i in 0..nums.len() / 2 {
        rez.push(nums[i]);
        rez.push(nums[nums.len() - i - 1]);
    }
    if nums.len() % 2 == 1 {
        rez.push(nums[nums.len() / 2]);
    }
    println!("DEBUGPRINT[1]: main.rs:12: rez={:#?}", rez);
}
