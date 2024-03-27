fn main() {
    println!("Hello, world!");
    let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
    // Output: 4
    println!("{}", longest_increasing_path(matrix));

    let matrix = vec![vec![0], vec![1], vec![5], vec![5]];
    // Output: 4
    println!("{}", longest_increasing_path(matrix));
}

pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut in_degree = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            // i and j are the froms
            for (di, dj) in &[(0, 1), (1, 0), (-1 as i32, 0), (0, -1 as i32)] {
                let ni = (i as i32 + di) as usize;
                let nj = (j as i32 + dj) as usize;
                if !(0..m).contains(&ni) || !(0..n).contains(&nj) {
                    continue;
                }
                if matrix[i][j] < matrix[ni][nj] {
                    in_degree[ni][nj] += 1;
                }
            }
        }
    }
    println!("{:?}", in_degree);
    let mut ready = std::collections::VecDeque::new();
    for i in 0..m {
        for j in 0..n {
            if in_degree[i][j] == 0 {
                ready.push_back((i, j));
            }
        }
    }
    let mut lvl = 0;
    while !ready.is_empty() {
        let size = ready.len();
        for _ in 0..size {
            let (i, j) = ready.pop_front().unwrap();
            // traverse the neighbors
            for (di, dj) in &[(0, 1), (1, 0), (-1 as i32, 0), (0, -1 as i32)] {
                let ni = (i as i32 + di) as usize;
                let nj = (j as i32 + dj) as usize;
                if !(0..m).contains(&ni) || !(0..n).contains(&nj) {
                    continue;
                }
                if matrix[i][j] < matrix[ni][nj] {
                    in_degree[ni][nj] -= 1;
                    if in_degree[ni][nj] == 0 {
                        ready.push_back((ni, nj));
                    }
                }
            }
        }
        lvl += 1;
        println!("ready={:?}", ready);
        println!("in_degree={:?}", in_degree);
    }
    lvl
}
