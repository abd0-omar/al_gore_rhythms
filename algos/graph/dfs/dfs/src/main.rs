use std::fs;

fn main() {
    println!("Hello, world!");

    let file = fs::read_to_string("input.txt").unwrap();

    let mut file = file.split_whitespace();

    let nodes: usize = file.next().unwrap().parse().unwrap();
    let edges: usize = file.next().unwrap().parse().unwrap();

    let mut graph: GRAPH = vec![Vec::new(); nodes];

    for _ in 0..edges {
        let from: usize = file.next().unwrap().parse().unwrap();
        let to: usize = file.next().unwrap().parse().unwrap();
        add_edge(&mut graph, from, to);
    }

    reachability(&graph);
}

type GRAPH = Vec<Vec<usize>>;

// [ [ v visited] , [] , [] , [b] ]
//   [b  , , , , , , ]
fn add_edge(graph: &mut GRAPH, from: usize, to: usize) {
    graph[from].push(to);
}

fn dfs(graph: &GRAPH, node: usize, visited: &mut Vec<bool>) {
    visited[node] = true;

    // println!(" node {} ", node);
    for &neighbor in &graph[node] {
        if visited[neighbor] != true {
            println!(" node {} ", neighbor);
            dfs(graph, neighbor, visited);
        }
    }
}

fn reachability(graph: &GRAPH) {
    for i in 0..graph.len() {
        let mut visited = vec![false; graph.len()];
        println!("{} can reach: ", i);
        dfs(graph, i, &mut visited);
        println!();
    }
}
