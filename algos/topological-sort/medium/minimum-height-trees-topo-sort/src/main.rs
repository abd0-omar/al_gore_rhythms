fn main() {
    println!("Hello, world!");
    let n = 4;
    let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3]];
    // Output: [1]
    println!("{:?}", find_min_height_trees(n, edges));

    let n = 6;
    let edges = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];
    println!("{:?}", find_min_height_trees(n, edges));
    // Output: [3,4]
}

pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    if n == 1 {
        return vec![0];
    }
    let mut graph = vec![vec![]; n as usize];
    let mut in_degree = vec![0; n as usize];

    for edge in edges.iter() {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        graph[from].push(to);
        graph[to].push(from);
        in_degree[from] += 1;
        in_degree[to] += 1;
    }
    println!("{:?}", graph);

    // put in the ready queue the leaves with in_degree of 1
    let mut ready = std::collections::VecDeque::new();
    for g in graph.iter() {
        for &node in g.iter() {
            if in_degree[node] == 1 {
                ready.push_back(node);
            }
        }
    }

    let mut graph_size = graph.len();
    while graph_size > 2 {
        let size = ready.len();
        graph_size -= size;
        for _ in 0..size {
            let cur = ready.pop_front().unwrap();
            for &neighbor in graph[cur].iter() {
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 1 {
                    ready.push_back(neighbor);
                }
            }
        }
    }
    ready.into_iter().map(|x| x as i32).collect()
}
