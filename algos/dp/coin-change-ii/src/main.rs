fn main() {
    println!("Hello, world!");
    let amount = 5;
    let coins = vec![1, 2, 5];
    println!("{}", change(amount, coins));
    let amount = 3;
    let coins = vec![2];
    println!("{}", change(amount, coins));
    // let amount = 10;
    // let coins = vec![10];
    // println!("{}", change(amount, coins));
}

pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut memory = std::collections::HashMap::new();
    _change(amount, &coins, 0, &mut res, &mut memory);
    res
}

pub fn _change(
    amount: i32,
    coins: &Vec<i32>,
    idx: usize,
    res: &mut i32,
    memory: &mut std::collections::HashMap<(usize, i32), i32>,
) {
    if idx == coins.len() || amount <= 0 {
        // println!("amount={:?}", amount);
        if amount == 0 {
            *res += 1;
        }
        return;
    }

    if let Some(&ret) = memory.get(&(idx, amount)) {
        println!("ret={:?}", ret);
        *res += ret;
        return;
    }

    // leave
    let leave_result = *res;
    _change(amount, coins, idx + 1, res, memory);

    // pick and stay
    _change(amount - coins[idx], coins, idx, res, memory);

    // Store the result in memory after making recursive calls
    memory.insert((idx, amount), *res - leave_result);
}
