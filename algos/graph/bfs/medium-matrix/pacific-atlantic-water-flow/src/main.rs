use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");

    let heights = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];
    // Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
    println!("{:?}", pacific_atlantic(heights));
}

// vector<vector<int>> pacificAtlantic(vector<vector<int>>& heights)

fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = heights.len();
    let columns = heights[0].len();

    let mut visited = vec![vec![false; columns]; rows];
    // pacific (0, 0) -> (0, n) && (0, 0) -> (n, 0)
    // atlantic (n , 0) -> (n, n) && (0, n) -> (n, n)
    // atlantic (n , n) -> (0, 0) && (0, 0) -> (n, 0)
    //
    //
    let mut pacific = vec![vec![false; columns]; rows];
    let mut atlantic = vec![vec![false; columns]; rows];
    //row
    for i in 0..rows {
        // bfs(&heights, &mut pacific, 0, i as i32);
        // bfs(&heights, &mut atlantic, columns as i32 - 1, i as i32);
        bfs(&heights, &mut pacific, i as i32, 0);
        bfs(&heights, &mut atlantic, i as i32, columns as i32 - 1);
    }

    println!("pacific={:#?}", pacific);
    println!("atlantic={:#?}", atlantic);

    for j in 0..columns {
        bfs(&heights, &mut pacific, 0, j as i32);
        bfs(&heights, &mut atlantic, (rows - 1) as i32, j as i32);
    }
    println!("pacific={:?}", pacific);
    println!("atlantic={:?}", atlantic);

    let mut chain = Vec::new();

    for i in 0..rows {
        for j in 0..columns {
            if pacific[i][j] == atlantic[i][j] {
                chain.push(vec![i as i32, j as i32]);
            }
        }
    }

    chain
}

fn bfs(heights: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, i: i32, j: i32) {
    let mut q = VecDeque::new();

    if (0..heights.len()).contains(&(i as usize)) && (0..heights[0].len()).contains(&(j as usize)) {
        q.push_back((i, j));
    }

    while !q.is_empty() {
        let (i, j) = q.pop_front().unwrap();
        visited[i as usize][j as usize] = true;

        for &(di, dj) in &[(1, 0), (0, -1), (-1 as i32, 0 as i32), (0, 1)] {
            let new_i = di + i;
            let new_j = dj + j;

            if (0..heights.len()).contains(&(new_i as usize))
                && (0..heights[0].len()).contains(&(new_j as usize))
                && !visited[new_i as usize][new_j as usize]
                && heights[new_i as usize][new_j as usize] >= heights[i as usize][j as usize]
            {
                q.push_back((new_i, new_j));
            }
        }
    }
}

// old dfs solution
// pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//     let mut chain: Vec<Vec<i32>> = Vec::new();
//     for i in 0..heights.len() {
//         for j in 0..heights[0].len() {
//             let mut visited = vec![vec![false; heights[0].len()]; heights.len()];
//             let old_cell = heights[i][j];
//             let mut top_left = false;
//             let mut bot_right = false;
//             let old_i = i;
//             let old_j = j;
//             dfs(
//                 &heights,
//                 &mut visited,
//                 i as i32,
//                 j as i32,
//                 old_cell,
//                 &mut chain,
//                 &mut top_left,
//                 &mut bot_right,
//                 old_i as i32,
//                 old_j as i32,
//                 -1,
//                 -1,
//             );
//             //println!("i={} , j={} ", i, j);
//         }
//     }
//     chain
// }
//
// fn is_valid(heights: &Vec<Vec<i32>>, i: i32, j: i32) -> bool {
//     if i < 0 || i >= heights.len() as i32 {
//         return false;
//     }
//     if j < 0 || j >= heights[0].len() as i32 {
//         return false;
//     }
//     true
// }
//
// fn dfs(
//     heights: &Vec<Vec<i32>>,
//     visited: &mut Vec<Vec<bool>>,
//     i: i32,
//     j: i32,
//     old_cell: i32,
//     chain: &mut Vec<Vec<i32>>,
//     top_left: &mut bool,
//     bot_right: &mut bool,
//     old_i: i32,
//     old_j: i32,
//     par_i: i32,
//     par_j: i32,
// ) {
//     if par_i == -1 {
//     } else if !is_valid(heights, i, j)
//         || visited[i as usize][j as usize]
//         || heights[i as usize][j as usize] > heights[par_i as usize][par_j as usize]
//         || (*top_left && *bot_right)
//     {
//         return;
//     }
//
//     if j == 0 || i == 0 {
//         *top_left = true;
//     }
//
//     if j == heights[0].len() as i32 - 1 || i == heights.len() as i32 - 1 {
//         *bot_right = true;
//     }
//
//     if *top_left && *bot_right {
//         chain.push(vec![old_i, old_j]);
//         return;
//     }
//
//     visited[i as usize][j as usize] = true;
//
//     let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];
//
//     for (add_i, add_j) in &directions {
//         let id = add_i + i;
//         let jd = add_j + j;
//
//         dfs(
//             heights, visited, id, jd, old_cell, chain, top_left, bot_right, old_i, old_j, i, j,
//         );
//     }
// }
