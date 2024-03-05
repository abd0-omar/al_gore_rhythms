fn main() {
    println!("Hello, world!");
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    println!("{}", min_cost_climbing_stairs(cost));
}

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut memory = vec![None; cost.len()];
    _min_cost_climbing_stairs(&cost, 0, &mut memory)
}

pub fn _min_cost_climbing_stairs(
    cost: &Vec<i32>,
    idx: usize,
    memory: &mut Vec<Option<i32>>,
) -> i32 {
    if idx == cost.len() {
        return 0;
    }
    if let Some(ret) = memory[idx] {
        return ret;
    }
    // 0 step
    let choice1 = cost[idx] + _min_cost_climbing_stairs(cost, idx + 1, memory);
    let mut choice2 = i32::MAX;
    if idx + 1 < cost.len() {
        choice2 = cost[idx + 1] + _min_cost_climbing_stairs(cost, idx + 2, memory);
    } else if idx + 1 == cost.len() {
        return 0;
        // choice2 = _min_cost_climbing_stairs(cost, idx + 1, memory);
    }
    // println!("choice1={:?}", choice1);
    // println!("choice2={:?}", choice2);
    memory[idx] = Some(choice1.min(choice2));

    choice1.min(choice2)
}
