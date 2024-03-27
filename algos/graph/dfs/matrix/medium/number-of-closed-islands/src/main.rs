fn main() {
    println!("Hello, world!");
    //     Example 1:
    //
    //
    //
    let grid = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 1, 1, 0],
        vec![1, 0, 1, 0, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 1, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 0],
    ];
    println!("{}", closed_island(grid));
    // Output: 2
    // Explanation:
    // Islands in gray are closed because they are completely surrounded by water (group of 1s).
    // Example 2:
    //
    //
    //
    let grid = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 1, 0, 1, 0],
        vec![0, 1, 1, 1, 0],
    ];
    println!("{}", closed_island(grid));
    // Output: 1
    // Example 3:
    //
    let grid = vec![
        vec![1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 1, 1, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 1, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1],
    ];
    println!("{}", closed_island(grid));
    // Output: 2
}

pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
    let dr = [-1, 0, 1, 0];
    let dc = [0, -1, 0, 1];

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut validation = true;
            if visited[i][j] || grid[i][j] == 1 {
                continue;
            }
            dfs(
                &grid,
                i as i32,
                j as i32,
                &mut visited,
                &dr,
                &dc,
                &mut validation,
            );
            if validation == true {
                count += 1;
            }
        }
    }
    // println!("DEBUGPRINT[1]: main.rs:52: visited={:?}", visited);
    count
}

fn is_valid(grid: &Vec<Vec<i32>>, i: i32, j: i32) -> bool {
    if i < 0 || i >= grid.len() as i32 {
        return false;
    }
    if j < 0 || j >= grid[0].len() as i32 {
        return false;
    }

    return true;
}

fn dfs(
    grid: &Vec<Vec<i32>>,
    i: i32,
    j: i32,
    visited: &mut Vec<Vec<bool>>,
    dr: &[i32; 4],
    dc: &[i32; 4],
    validation: &mut bool,
) {
    let iu = i as usize;
    let ju = j as usize;

    if !is_valid(grid, i, j) || visited[iu][ju] || grid[iu][ju] == 1 {
        return;
    }

    visited[iu][ju] = true;

    for d in 0..4 {
        let id = i + dr[d];
        let jd = j + dc[d];
        // if on edge, then not a close island
        if !is_valid(grid, id, jd) {
            *validation = false;
        }
        dfs(grid, id, jd, visited, dr, dc, validation);
    }
}
