fn main() {
    println!("Hello, world!");
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    // let word = "ABCCED".to_string();
    // Output: true
    // println!("{}", exist(board, word));

    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "SEE".to_string();
    // Output: true
    // println!("{}", exist(board, word));

    let board = vec![vec!['a']];
    let word = "a".to_string();
    // Output: true
    println!("{}", exist(board, word));

    let board = vec![vec!['a', 'b'], vec!['c', 'd']];
    let word = "acdb".to_string();
    println!("{}", exist(board, word));
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            let mut visited = vec![vec![false; board[0].len()]; board.len()];
            let x = _exist(
                &board,
                &word,
                i as i32,
                j as i32,
                String::new(),
                &mut visited,
            );
            if x {
                return true;
            }
        }
    }
    false
}

const DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn _exist(
    board: &Vec<Vec<char>>,
    word: &String,
    i: i32,
    j: i32,
    mut new_word: String,
    visited: &mut Vec<Vec<bool>>,
) -> bool {
    if &new_word == word {
        return true;
    }

    if new_word.len() > word.len() {
        return false;
    }

    new_word.push(board[i as usize][j as usize]);
    //println!("new_word={:#?}", new_word);
    visited[i as usize][j as usize] = true;

    if &new_word == word {
        return true;
    }

    for (di, dj) in DIR {
        let ni = di + i;
        let nj = dj + j;

        if (0..board.len()).contains(&(ni as usize)) && (0..board[0].len()).contains(&(nj as usize))
        {
            if !visited[ni as usize][nj as usize] {
                if _exist(board, word, ni, nj, new_word.clone(), visited) {
                    return true;
                }
            }
        }
    }

    new_word.pop();
    //println!("new_word={:#?}", new_word);
    visited[i as usize][j as usize] = false;

    &new_word == word
}
