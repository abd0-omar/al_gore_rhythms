fn main() {
    println!("Hello, world!");
    // Test Case 4: Shortest path to a different food is 4
    let matrix4: Vec<Vec<char>> = vec![
        vec!['X', 'X', 'X', 'X', 'X'],
        vec!['X', '*', 'O', 'O', 'X'],
        vec!['X', 'O', 'O', 'O', 'X'],
        vec!['X', 'X', 'X', '#', 'X'],
    ];
    let result4 = get_food(&matrix4);
    println!("result4={:?}", result4);
}
// int getFood(vector<vector<char>> &matrix)
fn get_food(matrix: &Vec<Vec<char>>) -> i32 {
    let mut len = vec![vec![None; matrix[0].len()]; matrix.len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '*' {
                return bfs(matrix, &mut len, i as i32, j as i32).unwrap_or(-1);
            }
        }
    }
    -1
}

fn is_valid(graph: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    (0..graph.len()).contains(&i) && (0..graph[0].len()).contains(&j)
}

#[derive(Debug)]
struct Cell(i32, i32);

fn bfs(
    matrix: &Vec<Vec<char>>,
    len: &mut Vec<Vec<Option<i32>>>,
    cur_i: i32,
    cur_j: i32,
) -> Option<i32> {
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back(Cell(cur_i, cur_j));
    let mut lvl = 0;
    len[cur_i as usize][cur_j as usize] = Some(lvl);
    let dr = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    while !q.is_empty() {
        let size = q.len();
        lvl += 1;
        for _ in 0..size {
            let Cell(cur_i, cur_j) = q.pop_front().unwrap();
            for (dr, dc) in dr {
                let id = dr + cur_i;
                let jd = dc + cur_j;
                if is_valid(&matrix, id as usize, jd as usize)
                    && len[id as usize][jd as usize].is_none()
                    && matrix[id as usize][jd as usize] != 'X'
                {
                    len[id as usize][jd as usize] = Some(lvl);
                    println!("id={:?}, jd={:?}", id, jd);
                    println!("matrix={:?}", matrix);
                    println!("len={:?}", len);
                    if matrix[id as usize][jd as usize] == '#' {
                        println!("lvl");
                        return Some(lvl);
                    }
                    q.push_back(Cell(id, jd));
                }
            }
        }
    }
    None
}
