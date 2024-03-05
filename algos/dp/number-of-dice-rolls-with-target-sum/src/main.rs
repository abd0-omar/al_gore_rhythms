use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    // let n = 2;
    // let k = 6;
    // let target = 7;
    // let n = 1;
    // let k = 6;
    // let target = 3;
    let n = 30;
    let k = 30;
    let target = 500;
    println!("{}", num_rolls_to_target(n, k, target));
}

pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    // no need for extra lis code
    // calc all possibilites: (1, 6) and (6, 1)
    // for i in 1..=k {
    //     sum += _num_rolls_to_target(n, i, k, target);
    // }
    let mut hm = HashMap::new();
    _num_rolls_to_target(n, 1, k, target, &mut hm)
}

const MOD: i32 = 1_000_000_007;

pub fn _num_rolls_to_target(
    n: i32,
    i: i32,
    k: i32,
    target: i32,
    hm: &mut HashMap<(i32, i32), i32>,
) -> i32 {
    if n == 0 {
        if target == 0 {
            return 1;
        }
        return 0;
    }

    if let Some(&ret) = hm.get(&(n, target)) {
        return ret;
    }

    let mut sum = 0;
    for j in i..=k {
        sum += _num_rolls_to_target(n - 1, i, k, target - j, hm);
        sum %= MOD;
    }

    hm.insert((n, target), sum);

    sum
}
