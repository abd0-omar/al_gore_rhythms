use std::collections::{HashSet, VecDeque};

fn main() {
    println!("Hello, world!");

    let board = vec![vec![1, 2, 3], vec![4, 5, 0]];
    println!("{}", sliding_puzzle(board));

    // Example 1:
    let board = vec![vec![1, 2, 3], vec![4, 0, 5]];
    // Output: 1
    // Explanation: Swap the 0 and the 5 in one move.
    // Example 2:
    println!("{}", sliding_puzzle(board));

    let board = vec![vec![1, 2, 3], vec![5, 4, 0]];
    // Output: -1
    // Explanation: No number of moves will make the board solved.
    // Example 3:
    println!("{}", sliding_puzzle(board));

    let board = vec![vec![4, 1, 2], vec![5, 0, 3]];
    // Output: 5
    // Explanation: 5 is the smallest number of moves that solves the board.
    // An example path:
    // After move 0: [[4,1,2],[5,0,3]]
    // After move 1: [[4,1,2],[0,5,3]]
    // After move 2: [[0,1,2],[4,5,3]]
    // After move 3: [[1,0,2],[4,5,3]]
    // After move 4: [[1,2,0],[4,5,3]]
    // After move 5: [[1,2,3],[4,5,0]
    println!("{}", sliding_puzzle(board));
}

pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
    let goal = vec![vec![1, 2, 3], vec![4, 5, 0]];
    if board == goal {
        return 0;
    }
    let mut q = VecDeque::new();
    q.push_back(board.clone());
    let mut visited = HashSet::new();
    visited.insert(board.clone());
    let mut lvl = 0;

    while !q.is_empty() {
        let size = q.len();
        lvl += 1;
        for _ in 0..size {
            let cur = q.pop_front().unwrap();
            for neighbor in get_neighbors(&cur, &mut visited) {
                if lvl == 19 {
                    return -1;
                }
                //check visited and push
                if neighbor == goal {
                    return lvl;
                }
                q.push_back(neighbor);
            }
        }
    }

    -1
}

fn get_neighbors(
    board: &Vec<Vec<i32>>,
    visited: &mut HashSet<Vec<Vec<i32>>>,
) -> Vec<Vec<Vec<i32>>> {
    let mut v = Vec::new();

    for i in 0..2 {
        for j in 0..3 {
            if board[i][j] == 0 {
                // println!("i={:?}", i);
                // println!("j={:?}", j);
                for &(di, dj) in &[(1, 0), (0, 1), (-1 as i32, 0), (0, -1 as i32)] {
                    let di = di + i as i32;
                    let dj = dj + j as i32;
                    // println!("di={:?}", di);
                    // println!("dj={:?}", dj);
                    if (0..2).contains(&di) && (0..3).contains(&dj) {
                        // println!("di={:?}", di);
                        // println!("dj={:?}", dj);
                        // modify the board
                        // println!("board={:?}", board);
                        let mut new_board = board.clone();

                        let temp = new_board[i][j]; //old_zero
                        new_board[i][j] = new_board[di as usize][dj as usize]; //replace old_zero with num
                                                                               // println!("new_board at temp={:?}", new_board);
                        new_board[di as usize][dj as usize] = temp; //put old_zero in place of num
                                                                    // println!("new_board after temp={:?}", new_board);
                                                                    // println!("new_board={:?}", new_board);
                        if visited.insert(new_board.clone()) {
                            v.push(new_board);
                        }

                        // new_board.swap((i, j), (di, dj));
                        // push the board
                    }
                }
            }
        }
    }

    // println!("v={:?}", v);
    v
}
