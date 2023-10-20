fn main() {
    //Input: difficulty = [2,4,6,8,10], profit = [10,20,30,40,50], worker = [4,5,6,7]
    // Output: 100
    // Explanation: Workers are assigned jobs of difficulty [4,4,6,6] and they get a profit of [20,20,30,30] separately.
    // Example 2:
    //
    // Input: difficulty = [85,47,57], profit = [24,66,99], worker = [40,25,25]
    // Output: 0
}

pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
    let (n, m) = (difficulty.len(), worker.len());
    let mut worker = worker;
    worker.sort_unstable();

    let mut pos = (0..n).collect::<Vec<usize>>();

    pos.sort_unstable_by(|&a, &b| difficulty[a].cmp(&difficulty[b]));
    println!("DEBUGPRINT[1]: main.rs:16: pos={:#?}", pos);

    let mut i = 0;
    let mut j = 0;

    let mut sum = 0;
    while i < m {
        let mut max_prof = 0;
        while j < n && worker[i] >= difficulty[pos[j]] {
            max_prof = std::cmp::max(profit[pos[i]], max_prof);
            j += 1;
        }
        sum += max_prof;
        i += 1;
    }

    sum
}
