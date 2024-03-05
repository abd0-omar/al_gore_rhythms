use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
    //     Example 1:
    //
    let n = 3;
    let blue_edges = vec![vec![0, 1], vec![1, 2]];
    let red_edges = vec![];
    println!(
        "ans= {:?}",
        shortest_alternating_paths(n, red_edges, blue_edges)
    );
    // Output: [0,1,-1]
    let n = 3;
    let red_edges = vec![vec![0, 1], vec![0, 2]];
    let blue_edges = vec![vec![1, 0]];
    println!(
        "ans= {:?}",
        shortest_alternating_paths(n, red_edges, blue_edges)
    );
    // Output: [0, 1, 1]
}

#[derive(Debug, Clone, PartialEq)]
enum Color {
    Red,
    Blue,
}

pub fn shortest_alternating_paths(
    n: i32,
    red_edges: Vec<Vec<i32>>,
    blue_edges: Vec<Vec<i32>>,
) -> Vec<i32> {
    let mut graph = vec![vec![]; n as usize];

    for r in red_edges {
        let from = r[0];
        let to = r[1];
        graph[from as usize].push((to as usize, Color::Red));
    }

    for b in blue_edges {
        let from = b[0];
        let to = b[1];
        graph[from as usize].push((to as usize, Color::Blue));
    }

    let mut q = VecDeque::new();
    q.push_back((0 as usize, Color::Red));
    q.push_back((0 as usize, Color::Blue));

    let mut len = vec![vec![-1; 2]; n as usize];
    len[0][Color::Red as usize] = 0;
    len[0][Color::Blue as usize] = 0;
    let mut lvl = 0;
    let mut res = vec![-1; n as usize];
    res[0] = 0;

    while !q.is_empty() {
        lvl += 1;
        let size = q.len();
        for _ in 0..size {
            let (cur, color) = q.pop_front().unwrap();

            for (neighbor, his_color) in &graph[cur] {
                if len[*neighbor][his_color.clone() as usize] == -1 && color != *his_color {
                    q.push_back((*neighbor, his_color.clone()));
                    println!("his_color={:?}", his_color);
                    len[*neighbor][his_color.clone() as usize] = lvl;
                    println!("len={:?}", his_color.clone() as usize); // Red as usize is 0
                    println!("len={:?}", his_color.clone() as usize); // Blue as usize is 1
                    if res[*neighbor] == -1 {
                        res[*neighbor] = lvl;
                    }
                }
            }
        }
    }

    res
}
