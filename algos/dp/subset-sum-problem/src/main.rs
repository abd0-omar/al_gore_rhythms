fn main() {
    let values = vec![3, 12, 4];
    let target = 9;
    println!("{}", subset_sum(&values, target));
    let values = vec![3, 40, 4, 12, 5, 2];
    let target = 30;
    println!("{}", subset_sum(&values, target));
    let values = vec![3, 12, 4];
    let target = 7;
    println!("{}", subset_sum(&values, target));
    let values = vec![3, 40, 4, 12, 5, 2];
    let target = 13;
    println!("{}", subset_sum(&values, target));
}

fn subset_sum(values: &Vec<i32>, target: i32) -> bool {
    _subset_sum(values, target, 0, target)
}

fn _subset_sum(values: &Vec<i32>, target: i32, idx: usize, prev_sum: i32) -> bool {
    if idx == values.len() {
        return prev_sum == 0;
    }

    // leave
    // don't take and go to anothe idx
    let choice1 = _subset_sum(values, target, idx + 1, prev_sum);
    let mut choice2 = false;

    if prev_sum - values[idx] >= 0 {
        // take the no. and go to another idx
        choice2 = _subset_sum(values, target, idx + 1, prev_sum - values[idx]);
    }

    choice1 || choice2
}
