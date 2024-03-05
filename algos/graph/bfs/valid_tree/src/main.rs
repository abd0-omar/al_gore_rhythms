use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
    // Example 1:

    // let n = 5;
    // let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]];
    // // Output: true.
    // // Example 2:
    //
    // println!("{}", valid_tree(n, edges));

    let n = 5;
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]];
    println!("{}", valid_tree(n, edges));
    // Output: false.
}

pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    if edges.len() as i32 != n - 1 {
        return false;
    }

    let mut graph: Vec<Vec<i32>> = vec![vec![]; n as usize];
    for i in 0..edges.len() {
        let from = edges[i][0];
        let to = edges[i][1];
        graph[from as usize].push(to as i32);
        graph[to as usize].push(from as i32);
    }

    let mut len: Vec<Option<i32>> = vec![None; n as usize];
    let mut parent = vec![None; n as usize];
    let mut validation = false;

    for i in 0..n as usize {
        println!("hello {i}");
        if (i != 0 && len[i].is_none()) || validation {
            // another component
            println!("{}", validation);
            println!("{i}");
            println!("hello");
            println!("parent={:?}", parent);
            println!("len={:?}", len);
            return false;
        }
        let cur = i as i32;
        if len[i].is_none() {
            bfs(&graph, &mut parent, &mut len, cur, &mut validation)
        }
    }

    true
}

fn bfs(
    graph: &Vec<Vec<i32>>,
    parent: &mut Vec<Option<i32>>,
    len: &mut Vec<Option<i32>>,
    cur: i32,
    validation: &mut bool,
) {
    let mut q: VecDeque<i32> = VecDeque::new();
    q.push_back(cur);
    let mut level = 0;
    len[cur as usize] = Some(level);
    println!("q={:?}", q);

    while !q.is_empty() {
        let size = q.len();
        level += 1;
        for _ in 0..size {
            let cur = q.pop_front().unwrap();
            for &neighbor in &graph[cur as usize] {
                if Some(neighbor) == parent[cur as usize] {
                    continue;
                }

                match len[neighbor as usize] {
                    Some(_) => {
                        *validation = true;
                        return;
                    }
                    None => {
                        // 1st time to visit
                        q.push_back(neighbor);
                        parent[neighbor as usize] = Some(cur);
                        len[neighbor as usize] = Some(level);
                    }
                }

                println!("q={:?}", q);
                println!("parent={:?}", parent);
                println!("len={:?}", len);
            }
        }
    }
}
