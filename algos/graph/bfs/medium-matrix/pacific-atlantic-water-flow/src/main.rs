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
    println!("pacific={:#?}", pacific);
    println!("atlantic={:#?}", atlantic);

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
