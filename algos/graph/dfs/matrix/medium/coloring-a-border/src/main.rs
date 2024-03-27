fn main() {
    println!("Hello, world!");
    //     Example 1:
    //
    // let grid = vec![vec![1, 1], vec![1, 2]];
    // let row = 0;
    // let col = 0;
    // let color = 3;
    // println!("{:?}", color_border(grid, row, col, color));
    // Output: vec![vec![3,3],vec![3,2]]
    // Example 2:
    //
    // let grid = vec![vec![1, 2, 2], vec![2, 3, 2]];
    // let row = 0;
    // let col = 1;
    // let color = 3;
    // println!("{:?}", color_border(grid, row, col, color));
    // Output: vec![vec![1,3,3],vec![2,3,3]]
    // Example 3:
    //
    let grid = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
    let row = 1;
    let col = 1;
    let color = 2;
    println!("{:?}", color_border(grid, row, col, color));
    // Output: [[2,2,2],[2,1,2],[2,2,2]]
}

pub fn color_border(mut grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let old_color = grid[row as usize][col as usize];
    dfs(&mut grid, row, col, &mut visited, old_color);
    create_boundries(&mut grid, &mut visited, color);
    grid
}

fn is_valid(grid: &Vec<Vec<i32>>, i: i32, j: i32) -> bool {
    if i < 0 || i >= grid.len() as i32 {
        return false;
    }
    if j < 0 || j >= grid[0].len() as i32 {
        return false;
    }
    true
}

// make every cell visisted using dfs, then see if it visited by all four dirs to not color it
// otherwise color it.
fn create_boundries(grid: &mut Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, color: i32) {
    let dr: [i32; 4] = [-1, 0, 1, 0];
    let dc: [i32; 4] = [0, -1, 0, 1];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !visited[i][j] {
                continue;
            }
            for d in 0..4 {
                if !is_valid(grid, i as i32 + dr[d], j as i32 + dc[d])
                    || !visited[(i as i32 + dr[d]) as usize][(j as i32 + dc[d]) as usize]
                {
                    grid[i][j] = color;
                    break;
                }
            }
        }
    }
}

fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32, visited: &mut Vec<Vec<bool>>, old_color: i32) {
    if !is_valid(grid, i, j)
        || visited[i as usize][j as usize]
        || grid[i as usize][j as usize] != old_color
    {
        return;
    }

    let dr = [-1, 0, 1, 0];
    let dc = [0, -1, 0, 1];
    // if i == 0 || j == 0 {
    //     println!("DEBUGPRINT[5]: main.rs:83: i={:?}, j={:?}", i, j);
    //     //color it
    //     grid[i as usize][j as usize] = color;
    // } else {
    //     let mut innit_bruv = false;
    //     for d in 0..4 {
    //         if !is_valid(grid, i + dr[d], j + dc[d]) {
    //             continue;
    //         }
    //         // if !visited[(i + dr[d]) as usize][(j + dc[d]) as usize] {
    //         if grid[(i + dr[d]) as usize][(j + dc[d]) as usize] != old_color {
    //             innit_bruv = true;
    //             break;
    //         }
    //     }
    //     if innit_bruv {
    //         //color it
    //         grid[i as usize][j as usize] = color;
    //     }
    // }

    visited[i as usize][j as usize] = true;

    for d in 0..4 {
        dfs(grid, i + dr[d], j + dc[d], visited, old_color);
    }
}
