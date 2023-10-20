fn main() {
    println!("Hello, world!");
    let ex1 = vec![1234, -234, 20, 2, 0, 0, -50000, 50000];
    println!("{:?}", sort_array(ex1));
}

fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    let mut freq: Vec<i32> = vec![0; 50000 + 50000 + 1];
    for e in nums.iter() {
        let x: usize = (*e + 50000) as usize;
        // eprintln!("DEBUGPRINT[2]: main.rs:13: x={:#?}", x);
        freq[x] += 1;
    }
    // -50000 -> 0
    // 0 -> 50000
    // 50000 ->
    let mut rez: Vec<i32> = Vec::with_capacity(nums.len());
    // for i in 0..freq.len() {
    //     while freq[i] > 0 {
    //         rez.push(i as i32 - 50000);
    //         freq[i] -= 1;
    //     }
    // }
    for (i, e) in freq.iter_mut().enumerate() {
        while *e > 0 {
            rez.push(i as i32 - 50000);
            *e -= 1;
        }
    }
    rez
}
