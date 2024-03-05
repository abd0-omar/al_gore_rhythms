fn main() {
    println!("Hello, world!");
    println!("{:?}", solve_n_queens(9));
}

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let board = vec![vec!['.'; n as usize]; n as usize];
    let mut final_board = vec![vec![]; 0];
    _solve_n_queens(n, &mut final_board, board, 0, 0, 0);
    println!("final_board={:?}", final_board);
    let f: Vec<Vec<String>> = final_board
        .iter()
        .map(|row| {
            row.iter()
                .map(|inside_row| inside_row.iter().collect::<String>())
                .collect()
        })
        .collect();
    f
}

pub fn _solve_n_queens(
    n: i32,
    final_board: &mut Vec<Vec<Vec<char>>>,
    mut curr_board: Vec<Vec<char>>,
    i: i32,
    j: i32,
    queens_added: i32,
) {
    // println!("i={:?}", i);
    // println!("j={:?}", j);
    // println!("curr_board={:?}", curr_board);
    if i == n && j == n {
        if queens_added == n {
            final_board.push(curr_board.clone());
        }
        return;
    }

    let ni;
    let nj;

    if j == n {
        ni = i + 1;
        nj = 0;
    } else {
        ni = i;
        nj = j;
    }

    // let ni = i;
    // let nj = j;
    // check if inbounds of board
    if (0..n).contains(&ni) && (0..n).contains(&nj) {
        // pick if no one sees it
        let mut no_one_sees_it = true;
        // rows/cols
        for j in 0..n {
            if curr_board[ni as usize][j as usize] == 'Q' {
                no_one_sees_it = false;
                break;
            }
        }

        if no_one_sees_it {
            for i in 0..n {
                if curr_board[i as usize][nj as usize] == 'Q' {
                    no_one_sees_it = false;
                    break;
                }
            }
        }

        // diags
        // main diag
        if no_one_sees_it {
            for i in 1..n {
                let new_idx_i = ni + i;
                let new_idx_j = nj + i;
                if !(0..n).contains(&new_idx_i) || !(0..n).contains(&new_idx_j) {
                    break;
                }
                if curr_board[new_idx_i as usize][new_idx_j as usize] == 'Q' {
                    no_one_sees_it = false;
                }
            }
        }

        if no_one_sees_it {
            for i in (1..=n - 1).rev() {
                let new_idx_i = ni - i;
                let new_idx_j = nj - i;
                // println!("points=({}, {})", new_idx_i, new_idx_j);
                // if ni == 1 && nj == 2 {
                //     println!("i={:?}", i);
                //     println!("new_idx_i={:?}", new_idx_i);
                //     println!("new_idx_j={:?}", new_idx_j);
                // }
                if !(0..n).contains(&new_idx_i) || !(0..n).contains(&new_idx_j) {
                    continue;
                }
                if curr_board[new_idx_i as usize][new_idx_j as usize] == 'Q' {
                    no_one_sees_it = false;
                }
            }

            // other diag
            for i in 1..n {
                let new_idx_i = ni + i;
                let new_idx_j = nj - i;
                if !(0..n).contains(&new_idx_i) || !(0..n).contains(&new_idx_j) {
                    break;
                }
                if curr_board[new_idx_i as usize][new_idx_j as usize] == 'Q' {
                    no_one_sees_it = false;
                }
            }
        }
        if no_one_sees_it {
            for i in 1..n {
                let new_idx_i = ni - i;
                let new_idx_j = nj + i;
                if !(0..n).contains(&new_idx_i) || !(0..n).contains(&new_idx_j) {
                    break;
                }
                if curr_board[new_idx_i as usize][new_idx_j as usize] == 'Q' {
                    no_one_sees_it = false;
                }
            }
        }

        if no_one_sees_it {
            // pick
            curr_board[ni as usize][nj as usize] = 'Q';
            _solve_n_queens(
                n,
                final_board,
                curr_board.clone(),
                ni,
                nj + 1,
                queens_added + 1,
            );
            curr_board[ni as usize][nj as usize] = '.';
        }
    }

    // leave
    _solve_n_queens(n, final_board, curr_board.clone(), ni, nj + 1, queens_added);
}
