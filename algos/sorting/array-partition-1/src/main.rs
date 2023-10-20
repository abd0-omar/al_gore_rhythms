fn main() {
    let mut input = vec![1, 2, 2, 5, 6, 6];
    println!("{}", array_pair_sum(&mut input));
}

fn array_pair_sum(nums: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    nums.sort_unstable();
    for num in nums.iter().step_by(2) {
        println!("DEBUGPRINT[1]: main.rs:8: num={:#?}", num);
        sum += *num;
    }
    sum
}
