fn main() {
    println!("Hello, world!");
    let m = 8;
    let n = 50;
    let max_move = 23;
    let start_row = 5;
    let start_column = 26;
    // Output: 6
    println!("{}", find_paths(m, n, max_move, start_row, start_column));
}

pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    let mut memory = std::collections::HashMap::new();
    _find_paths(m, n, max_move, start_row, start_column, &mut memory)
}

pub fn _find_paths(
    m: i32,
    n: i32,
    max_move: i32,
    i: i32,
    j: i32,
    memory: &mut std::collections::HashMap<(i32, i32, i32), i32>,
) -> i32 {
    if !(0..m).contains(&i) || !(0..n).contains(&j) {
        return 1;
    }

    if max_move == 0 {
        return 0;
    }

    if let Some(&ret) = memory.get(&(max_move, i, j)) {
        return ret;
    }

    let mut result = 0;
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for (dx, dy) in directions.iter() {
        let ni = i + dx;
        let nj = j + dy;
        result = (result + _find_paths(ni, nj, max_move - 1, m, n, memory)) % 1_000_000_007;
    }

    memory.insert((max_move, i, j), result);
    result
}
