fn main() {
    let ex = vec![3, -1];
    println!("{:?}", sort_array(ex));
}

fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    let min = nums.iter().min().unwrap(); // 49500
    let max = nums.iter().max().unwrap(); // 50000

    println!("DEBUGPRINT[1]: main.rs:6: min={:#?}", min);
    println!("DEBUGPRINT[2]: main.rs:8: max={:#?}", max);

    let mut freq = vec![0; (max - min + 1) as usize];
    println!("{}", max - min + 1);

    // 49500 -> 0
    // 0 -> 49500
    for e in nums.iter() {
        println!("{e}");
        println!("{}", (*e - min));
        freq[(*e - min) as usize] += 1;
    }

    let mut rez: Vec<i32> = Vec::with_capacity(nums.len());

    for (i, e) in freq.iter_mut().enumerate() {
        while *e > 0 {
            rez.push(i as i32 + min);
            *e -= 1;
        }
    }

    rez
}
