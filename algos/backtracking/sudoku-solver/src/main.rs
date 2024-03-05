fn main() {
    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    solve_sudoku(&mut board);
    println!("board={:?}", board);
}

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut curr_board = board.clone();
    _solve_sudoku(board, &mut curr_board, 0);
}

pub fn _solve_sudoku(board: &mut Vec<Vec<char>>, curr_board: &mut Vec<Vec<char>>, idx: usize) {
    if idx == 81 {
        *board = curr_board.clone();
        return;
    }

    let row = idx / 9;
    let col = idx % 9;

    if curr_board[row][col] != '.' {
        return _solve_sudoku(board, curr_board, idx + 1);
    }

    for num in '1'..='9' {
        if check(curr_board, row, col, num) {
            curr_board[row][col] = num;
            _solve_sudoku(board, curr_board, idx + 1);
            curr_board[row][col] = '.';
        }
    }
}

fn check(board: &Vec<Vec<char>>, row: usize, col: usize, num: char) -> bool {
    // Check row and column
    for i in 0..9 {
        if board[row][i] == num || board[i][col] == num {
            return false;
        }
    }

    // Check current 3x3 box
    let st_row = (row / 3) * 3;
    let st_col = (col / 3) * 3;
    for i in st_row..(st_row + 3) {
        for j in st_col..(st_col + 3) {
            if board[i][j] == num {
                return false;
            }
        }
    }
    true
}
