fn main() {
    let difficulty = vec![2, 4, 6, 8, 10];
    let profit = vec![10, 20, 30, 40, 50];
    let worker = vec![4, 5, 6, 7];

    let difficulty = vec![85, 47, 57];
    let profit = vec![24, 66, 99];
    let worker = vec![40, 25, 25];

    let difficulty = vec![13, 37, 58];
    let profit = vec![4, 90, 96];
    let worker = vec![34, 73, 45];

    println!("{}", max_profit_assignment(difficulty, profit, worker));
    // Output: 100
    // Explanation: Workers are assigned jobs of difficulty [4,4,6,6] and they get a profit of [20,20,30,30] separately.
}

pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
    let mut sum = 0;

    //get highest profit that the worker with certain diff can get

    let mut diff_pairs: Vec<(i32, i32)> = Vec::with_capacity(difficulty.len());
    for i in 0..difficulty.len() {
        let x = (difficulty[i], profit[i]);
        diff_pairs.push(x);
    }

    diff_pairs.sort_by_key(|&pair| pair.0);

    for i in 0..worker.len() {
        let mut end: i32 = -1;
        for j in 0..difficulty.len() {
            if diff_pairs[j].0 > worker[i] {
                // highest_prof = std::cmp::max(highest_prof, profit[j]);
                if j == 0 {
                    end = -2;
                    break;
                } else {
                    end = j as i32 - 1;
                    break;
                }
            }
        }
        let mut highest_prof = 0;
        if end == -1 {
            //nothing is difficult for him
            end = difficulty.len() as i32 - 1;
            for j in 0..=end {
                highest_prof = std::cmp::max(highest_prof, diff_pairs[j as usize].1);
            }
        } else if end == -2 {
            //everything is difficult for him
            highest_prof = 0;
            end = 0;
        } else {
            for j in 0..=end {
                highest_prof = std::cmp::max(highest_prof, diff_pairs[j as usize].1);
            }
        }
        sum += highest_prof;
    }
    sum
}

pub fn max_profit_assignment_with_debug_prints(
    difficulty: Vec<i32>,
    profit: Vec<i32>,
    worker: Vec<i32>,
) -> i32 {
    let mut sum = 0;

    //get highest profit that the worker with certain diff can get

    let mut diff_pairs: Vec<(i32, i32)> = Vec::with_capacity(difficulty.len());
    for i in 0..difficulty.len() {
        let x = (difficulty[i], profit[i]);
        diff_pairs.push(x);
    }

    diff_pairs.sort_by_key(|&pair| pair.0);
    println!("DEBUGPRINT[11]: main.rs:30: diff_pairs={:#?}", diff_pairs);

    for i in 0..worker.len() {
        let mut end: i32 = -1;
        for j in 0..difficulty.len() {
            if diff_pairs[j].0 > worker[i] {
                // highest_prof = std::cmp::max(highest_prof, profit[j]);
                if j == 0 {
                    end = -2;
                    break;
                } else {
                    end = j as i32 - 1;
                    break;
                }
            }
        }
        let mut highest_prof = 0;
        println!("DEBUGPRINT[7]: main.rs:40: end={:#?}", end);
        if end == -1 {
            //nothing is difficult for him
            end = difficulty.len() as i32 - 1;
            for j in 0..=end {
                highest_prof = std::cmp::max(highest_prof, diff_pairs[j as usize].1);
            }
        } else if end == -2 {
            //everything is difficult for him
            highest_prof = 0;
            end = 0;
        } else {
            for j in 0..=end {
                highest_prof = std::cmp::max(highest_prof, diff_pairs[j as usize].1);
                println!("DEBUGPRINT[10]: main.rs:59: end={:#?}", end);
                println!(
                    "DEBUGPRINT[8]: main.rs:45: profit={:#?}",
                    diff_pairs[j as usize].1
                );
                println!(
                    "DEBUGPRINT[9]: main.rs:53: highest_prof={:#?}",
                    highest_prof
                );
            }
        }
        println!("DEBUGPRINT[6]: main.rs:43: sum={:#?}", sum);
        sum += highest_prof;
        println!("DEBUGPRINT[6]: main.rs:43: sum={:#?}", sum);
    }
    sum
}
