fn main() {
    let n = 4; // Example value for n
    let result = solve_n_queens(n);
    println!("{:?}", result);
}

fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    let mut solutions = Vec::new();
    let mut columns = vec![false; n];
    let mut norm_diag = vec![false; 2 * n - 1];
    let mut anti_diag = vec![false; 2 * n - 1];
    let empty_grid = vec![".".repeat(n); n];

    backtrack(
        0,
        &mut solutions,
        &mut columns,
        &mut norm_diag,
        &mut anti_diag,
        &empty_grid,
    );

    solutions
}

fn backtrack(
    r: usize,
    solutions: &mut Vec<Vec<String>>,
    columns: &mut Vec<bool>,
    norm_diag: &mut Vec<bool>,
    anti_diag: &mut Vec<bool>,
    empty_grid: &Vec<String>,
) {
    let n = columns.len();

    if r == n {
        solutions.push(empty_grid.clone());
        return;
    }

    for c in 0..n {
        let x = r + c;
        let y = n - 1 + r - c;

        println!("columns={:?}", columns);
        println!("norm_diag={:?}", norm_diag);
        if columns[c] || norm_diag[y as usize] || anti_diag[x] {
            continue;
        }

        let mut row: Vec<char> = empty_grid[r].chars().collect();
        row[c] = 'Q';
        let mut grid = empty_grid.clone();
        grid[r] = row.iter().collect();

        columns[c] = true;
        norm_diag[y as usize] = true;
        anti_diag[x] = true;

        backtrack(r + 1, solutions, columns, norm_diag, anti_diag, &grid);

        columns[c] = false;
        norm_diag[y as usize] = false;
        anti_diag[x] = false;
    }
}
