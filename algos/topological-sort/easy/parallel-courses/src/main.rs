use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

// ● C++: int minimumSemesters(int n, vector<vector<int>>& relations)

fn minimum_semesters(n: i32, relations: &Vec<Vec<i32>>) -> Option<i32> {
    let n = n as usize;
    //      1
    // 1 -> 3
    // 2 -> 3
    let mut indegree = vec![0; n];
    let mut graph = vec![vec![]; n];

    // calculate indegree + make graph
    for i in 0..relations.len() {
        let from = relations[i][0] as usize;
        let to = relations[i][1];
        graph[from].push(to);
        indegree[to as usize] += 1;
    }

    // put zeros in q

    let mut q = VecDeque::new();

    for (node, &node_degree) in indegree.iter().enumerate() {
        if node_degree == 0 {
            q.push_back(node);
        }
    }

    // nodes_no. instead of ordered vec
    let mut found_nodes = 0;

    let mut lvl = 0;
    while !q.is_empty() {
        lvl += 1;
        let size = q.len();
        for _ in 0..size {
            let node = q.pop_front().unwrap();
            found_nodes += 1;
            for &neighbor in &graph[node] {
                indegree[neighbor as usize] -= 1;
                if indegree[neighbor as usize] == 0 {
                    q.push_back(neighbor as usize);
                }
            }
        }
    }

    if found_nodes == n {
        return Some(lvl);
    }

    None
}
