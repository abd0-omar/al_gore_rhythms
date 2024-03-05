fn main() {
    println!("Hello, world!");
    let coins = vec![2, 3, 5];
    let amount = 9;
    println!("{}", coin_change(coins, amount));
}

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut memory = std::collections::HashMap::new();
    match _coin_change(&coins, amount, 0, &mut memory) {
        Some(val) => val,
        None => -1,
    }
}

fn _coin_change(
    coins: &Vec<i32>,
    amount: i32,
    idx: usize,
    memory: &mut std::collections::HashMap<(usize, i32), Option<i32>>,
) -> Option<i32> {
    if amount == 0 {
        return Some(0);
    }

    if amount < 0 || idx == coins.len() {
        return None;
    }

    if let Some(ret) = memory.get(&(idx, amount)) {
        return *ret;
    }

    // let mut choice1 = None;
    // let mut choice2 = None;
    //
    // // pick and stay
    // // (1 +) and (- coins[idx]) means pick
    // // idx means stay
    // if let Some(val) = _coin_change(coins, amount - coins[idx], idx, memory) {
    //     choice1 = Some(1 + val);
    // }
    //
    // // leave
    // // idx + 1 means leave
    // if let Some(val) = _coin_change(coins, amount, idx + 1, memory) {
    //     choice2 = Some(val);
    // }

    // leave
    let choice1 = _coin_change(coins, amount, idx + 1, memory);
    // pick and stay on the same idx
    let choice2 = _coin_change(coins, amount - coins[idx], idx, memory).map(|val| val + 1);

    // memory[idx][amount as usize] = match (choice1, choice2) {
    //     (None, None) => None,
    //     (None, Some(_)) => choice2,
    //     (Some(_), None) => choice1,
    //     (Some(val1), Some(val2)) => Some(val1.min(val2)),
    // };
    // memory[idx][amount as usize]

    let result = match (choice1, choice2) {
        (None, None) => None,
        (None, Some(_)) => choice2,
        (Some(_), None) => choice1,
        (Some(val1), Some(val2)) => Some(val1.min(val2)),
    };

    memory.insert((idx, amount), result);
    result
}
