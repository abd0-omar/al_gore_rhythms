fn main() {
    // println!("Hello, world!");
    // //     Example 1:
    // //
    // //
    // //
    // let grid = vec![
    //     vec!['a', 'a', 'a', 'a'],
    //     vec!['a', 'b', 'b', 'a'],
    //     vec!['a', 'b', 'b', 'a'],
    //     vec!['a', 'a', 'a', 'a'],
    // ];
    // println!("{}", contains_cycle(grid));
    // // Output: true
    // // Explanation: There are two valid cycles shown in different colors in the image below:
    // //
    // // Example 2:
    // //
    // //
    // //
    // let grid = vec![
    //     vec!['c', 'c', 'c', 'a'],
    //     vec!['c', 'd', 'c', 'c'],
    //     vec!['c', 'c', 'e', 'c'],
    //     vec!['f', 'c', 'c', 'c'],
    // ];
    // println!("{}", contains_cycle(grid));
    // // Output: true
    // // Explanation: There is only one valid cycle highlighted in the image below:
    // //
    // // Example 3:
    // //
    // //
    // //
    // let grid = vec![
    //     vec!['a', 'b', 'b'],
    //     vec!['b', 'z', 'b'],
    //     vec!['b', 'b', 'a'],
    // ];
    // println!("{}", contains_cycle(grid));
    // // Output: false
    // // Input
    // let grid = vec![
    //     vec!['b', 'a', 'c'],
    //     vec!['c', 'a', 'c'],
    //     vec!['d', 'd', 'c'],
    //     vec!['b', 'c', 'c'],
    // ];
    // println!("{}", contains_cycle(grid));
    // //
    // // Use Testcase
    // // Output
    // // true
    // // Expected
    // // false
    // //     Input
    // let grid = vec![
    //     vec!['d', 'b', 'b'],
    //     vec!['c', 'a', 'a'],
    //     vec!['b', 'a', 'c'],
    //     vec!['c', 'c', 'c'],
    //     vec!['d', 'd', 'a'],
    // ];
    // println!("{}", contains_cycle(grid));
    //
    // Use Testcase
    // Output
    // true
    // Expected
    // false
    //
    let grid = vec![
        vec!['c', 'a', 'd'],
        vec!['a', 'a', 'a'],
        vec!['a', 'a', 'd'],
        vec!['a', 'c', 'd'],
        vec!['a', 'b', 'c'],
    ];
    println!("{}", contains_cycle(grid));
}

pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r as usize][c as usize] {
                let mut is_cycle = false;
                dfs(
                    r,
                    c,
                    &grid,
                    &mut visited,
                    grid[r as usize][c as usize],
                    -1,
                    -1,
                    &mut is_cycle,
                );
                if is_cycle {
                    return true;
                }
            }
        }
    }
    false
}

fn is_valid(i: i32, j: i32, grid: &Vec<Vec<char>>) -> bool {
    if i < 0 || i >= grid.len() as i32 {
        return false;
    }
    if j < 0 || j >= grid[0].len() as i32 {
        return false;
    }
    true
}

fn dfs(
    r: i32,
    c: i32,
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    old_color: char,
    par_r: i32,
    par_c: i32,
    is_cycle: &mut bool,
) {
    if !is_valid(r, c, grid) || grid[r as usize][c as usize] != old_color || *is_cycle {
        return;
    }

    if visited[r as usize][c as usize] {
        *is_cycle = true;
        return;
    }

    visited[r as usize][c as usize] = true;

    let dr = [-1, 0, 1, 0];
    let dc = [0, 1, 0, -1];

    for d in 0..4 {
        let nr = r + dr[d];
        let nc = c + dc[d];

        if nr == par_r && nc == par_c {
            continue;
        }

        dfs(nr, nc, grid, visited, old_color, r, c, is_cycle);
    }
}
