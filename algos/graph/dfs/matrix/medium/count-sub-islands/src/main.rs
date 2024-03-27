fn main() {
    let grid1 = vec![
        vec![1, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 0, 1, 1],
    ];
    let grid2 = vec![
        vec![1, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 1],
        vec![0, 1, 0, 0, 0],
        vec![1, 0, 1, 1, 0],
        vec![0, 1, 0, 1, 0],
    ];

    let grid1 = vec![
        vec![1, 0, 1, 0, 1],
        vec![1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1],
        vec![1, 0, 1, 0, 1],
    ];
    let grid2 = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1],
        vec![0, 1, 0, 1, 0],
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 0, 0, 1],
    ];
    println!("{}", count_sub_islands(grid1, grid2));
}

// ● int countSubIslands(vector<vector<int>> &grid1, vector<vector<int>> &grid2)

fn count_sub_islands(mut grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
    // let mut graph = V
    let m = grid1.len();
    let n = grid1[0].len();
    let mut visited = vec![vec![false; n]; m];
    let mut count = 0;
    for i in 0..m {
        for j in 0..n {
            if grid2[i][j] == 1 && !visited[i][j] {
                let mut validation = true;
                dfs(
                    &mut grid1,
                    &mut grid2,
                    &mut visited,
                    i as i32,
                    j as i32,
                    m,
                    n,
                    &mut validation,
                );
                if validation {
                    count += 1;
                }
            }
        }
    }
    count
}

fn is_valid(i: i32, j: i32, m: usize, n: usize) -> bool {
    if i < 0 || i >= m as i32 {
        // println!("DEBUGPRINT[2]: main.rs:51: i={:?}", i);
        return false;
    }

    if j < 0 || j >= n as i32 {
        return false;
    }
    true
}

fn dfs(
    grid1: &mut Vec<Vec<i32>>,
    grid2: &mut Vec<Vec<i32>>,
    visited: &mut Vec<Vec<bool>>,
    i: i32,
    j: i32,
    m: usize,
    n: usize,
    validation: &mut bool,
) {
    // *count += 1;

    println!("DEBUGPRINT[1]: main.rs:73: i={:?}, j={:?}", i, j);
    println!(
        "DEBUGPRINT[3]: main.rs:75: is_valid={:?}",
        is_valid(1, j, m, n)
    );
    if !is_valid(i, j, m, n) || visited[i as usize][j as usize]
    /* || grid2[i][j] == 0 || grid1[i][j] == 0  */
    || grid2[i as usize][j as usize] == 0
    {
        return;
    }

    if grid1[i as usize][j as usize] == 0
    /* && grid1[i][j] == 1  */
    {
        *validation = false;
        return;
    }

    visited[i as usize][j as usize] = true;

    let dr: [i32; 4] = [-1, 0, 1, 0];
    let dc: [i32; 4] = [0, -1, 0, 1];

    for d in 0..4 {
        println!("new");
        dfs(
            grid1,
            grid2,
            visited,
            i + dr[d],
            j + dc[d],
            m,
            n,
            validation,
        );
    }
}
