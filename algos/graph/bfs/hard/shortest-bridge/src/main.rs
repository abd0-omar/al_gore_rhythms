fn main() {
    println!("Hello, world!");
    let mut count = 0;
    // Example 1:

    let grid = vec![vec![0, 1], vec![1, 0]];
    // Output: 1
    // Example 2:
    println!("{count}: {}", shortest_bridge(grid));
    count += 1;

    let grid = vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]];
    // Output: 2
    // Example 3:
    println!("{count}: {}", shortest_bridge(grid));
    count += 1;

    let grid = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1],
    ];
    // Output: 1
    println!("{count}: {}", shortest_bridge(grid));
    count += 1;

    let grid = vec![
        vec![0, 1, 0, 0, 0],
        vec![0, 1, 0, 1, 1],
        vec![0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];
    // Output: 1

    println!("{count}: {}", shortest_bridge(grid));
}

pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
    let mut q = std::collections::VecDeque::new();
    let mut first_visited = std::collections::HashSet::new();

    // search for any 1
    'outer: for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                q.push_back((i as i32, j as i32));
                first_visited.insert((i as i32, j as i32));
                break 'outer;
            }
        }
    }

    // now make it all 1s connected to it visited

    while let Some((i, j)) = q.pop_front() {
        for &(di, dj) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let i = i + di;
            let j = j + dj;

            if (0..grid.len()).contains(&(i as usize))
                && (0..grid[0].len()).contains(&(j as usize))
                && grid[i as usize][j as usize] == 1
                && !first_visited.contains(&(i, j))
            {
                first_visited.insert((i, j));
                q.push_back((i, j));
            }
        }
    }

    println!("first_visited={:?}", first_visited);

    // search for the other 1 and put it in the queue
    let mut second_visited = std::collections::HashSet::new();
    let mut lvl = -1;

    //push all ones
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !first_visited.contains(&(i as i32, j as i32)) && grid[i][j] == 1 {
                q.push_back((i as i32, j as i32));
                second_visited.insert((i as i32, j as i32));
            }
        }
    }

    // now make it all 1s connected to it visited
    // while let Some((i, j)) = q.pop_front() {
    //     for &(di, dj) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
    //         let i = i + di;
    //         let j = j + dj;
    //
    //         if (0..grid.len()).contains(&(i as usize))
    //             && (0..grid[0].len()).contains(&(j as usize))
    //             && grid[i as usize][j as usize] == 1
    //             && !second_visited.contains(&(i, j))
    //         {
    //             second_visited.insert((i, j));
    //             q.push_back((i, j));
    //         }
    //     }
    // }

    println!("second_visited={:?}", second_visited);
    // q.push_back(second_one);
    println!("q={:?}", q);

    while !q.is_empty() {
        lvl += 1;
        let size = q.len();
        for _ in 0..size {
            let (i, j) = q.pop_front().unwrap();

            for &(di, dj) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let i = i + di;
                let j = j + dj;
                if (0..grid.len()).contains(&(i as usize))
                    && (0..grid[0].len()).contains(&(j as usize))
                    && !second_visited.contains(&(i, j))
                {
                    if grid[i as usize][j as usize] == 1 {
                        return lvl;
                    }

                    second_visited.insert((i, j));
                    q.push_back((i, j));
                }
            }
        }
    }

    unreachable!()
}
