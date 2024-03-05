use std::collections::VecDeque;

const INF: i32 = 2147483647;
fn main() {
    println!("Hello, world!");
    let mut rooms = vec![
        vec![INF, -1, 0, INF],
        vec![INF, INF, INF, -1],
        vec![INF, -1, INF, -1],
        vec![0, -1, INF, INF],
    ];
    walss_and_gates(&mut rooms);
    println!("{:?}", rooms);
}
//rooms=[[2147483647, -1, 0, 2147483647], [2147483647, 2147483647, 1, -1], [2147483647, -1, 2147483647, -1], [0, -1, 2147483647, 2147483647]]
// rooms=[[2147483647, -1, 0, 1], [2147483647, 2147483647, 1, -1], [2147483647, -1, 2147483647, -1], [0, -1, 2147483647, 2147483647]]
// rooms=[[2147483647, -1, 0, 1], [2147483647, 2147483647, 1, -1], [1, -1, 2147483647, -1], [0, -1, 2147483647, 2147483647]]
// rooms=[[2147483647, -1, 0, 1], [2147483647, 2147483647, 1, -1], [1, -1, 2, -1], [0, -1, 2147483647, 2147483647]]
// rooms=[[2147483647, -1, 0, 1], [2147483647, 2, 1, -1], [1, -1, 2, -1], [0, -1, 2147483647, 2147483647]]
// rooms=[[2147483647, -1, 0, 1], [2, 2, 1, -1], [1, -1, 2, -1], [0, -1, 2147483647, 2147483647]]
// rooms=[[2147483647, -1, 0, 1], [2, 2, 1, -1], [1, -1, 2, -1], [0, -1, 3, 2147483647]]
// rooms=[[3, -1, 0, 1], [2, 2, 1, -1], [1, -1, 2, -1], [0, -1, 3, 2147483647]]
// rooms=[[3, -1, 0, 1], [2, 2, 1, -1], [1, -1, 2, -1], [0, -1, 3, 4]]

// Given the 2D grid:
//
// INF  -1  0  INF
// INF INF INF  -1
// INF  -1 INF  -1
//   0  -1 INF INF
// After running your function, the 2D grid should be:
//
//   3  -1   0   1
//   2   2   1  -1
//   1  -1   2  -1
//   0  -1   3   4

// void wallsAndGates(vector<vector<int>>& rooms)
fn walss_and_gates(rooms: &mut Vec<Vec<i32>>) {
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let mut visited = vec![vec![false; rooms[0].len()]; rooms.len()];

    //insert the two gates in the queue to make them run with each other
    for i in 0..rooms.len() {
        for j in 0..rooms[0].len() {
            if rooms[i][j] == 0 {
                q.push_back((i as i32, j as i32));
                visited[i][j] = true;
            }
        }
    }

    println!("q={:?}", q); // i,j of the two gates

    let mut lvl = 0;

    while !q.is_empty() {
        lvl += 1;
        let size = q.len();
        for _ in 0..size {
            let (i, j) = q.pop_front().unwrap();
            for &(di, dj) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let di = di + i;
                let dj = dj + j;
                if (0..rooms.len()).contains(&(di as usize))
                    && (0..rooms[0].len()).contains(&(dj as usize))
                    && !visited[di as usize][dj as usize]
                    && rooms[di as usize][dj as usize] != -1
                {
                    q.push_back((di, dj));
                    visited[di as usize][dj as usize] = true;
                    rooms[di as usize][dj as usize] = lvl;
                    println!("rooms={:#?}", rooms);
                }
            }
        }
    }
}
