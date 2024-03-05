fn main() {
    println!("Hello, world!");
    let n = 6;
    let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
    // Output: 3
    println!("{}", min_reorder(n, &connections));
}
// int minReorder(int nodes, vector<vector<int>> &connections)

fn min_reorder(nodes: i32, connections: &Vec<Vec<i32>>) -> i32 {
    let mut graph_undirected: Vec<Vec<i32>> = vec![vec![]; nodes as usize];
    let mut graph_directed: Vec<Vec<i32>> = vec![vec![]; nodes as usize];
    let mut graph_inverse: Vec<Vec<i32>> = vec![vec![]; nodes as usize];

    for i in 0..(nodes - 1) as usize {
        let from = connections[i][0] as usize;
        let to = connections[i][1] as usize;
        graph_undirected[from].push(to as i32);
        graph_undirected[to].push(from as i32);
        graph_directed[from].push(to as i32);
        graph_inverse[to].push(from as i32);
    }
    println!("graph_undirected={:?}", graph_undirected);
    println!("graph_directed= {:?}", graph_directed);
    println!("graph_inverse=   {:?}", graph_inverse);
    let mut visited = vec![false; nodes as usize];
    for i in 0..nodes as usize {
        if !visited[i] {
            dfs(&graph_undirected, &graph_inverse, i as i32, &mut visited);
        }
    }
    println!("graph_undirected={:?}", graph_undirected);
    println!("graph_inverse=   {:?}", graph_inverse);
    unimplemented!()
}

// 0 -> 1
// if ! 1 -> 0
// count += 1
fn dfs(
    graph_undirected: &Vec<Vec<i32>>,
    graph_inverse: &Vec<Vec<i32>>,
    node: i32,
    visited: &mut Vec<bool>,
) {
    if visited[node as usize] == true {
        return;
    }

    // if graph_directed[node as usize]

    visited[node as usize] = true;

    for &neighbor in &graph_undirected[node as usize] {
        // [1, 4], [0, 3] undirected
        // [1],    [3]  directed
        // [4],    [0] inverse directed
        // neighbor -> 1
        //
        dfs(graph_undirected, graph_inverse, neighbor, visited)
    }
}
