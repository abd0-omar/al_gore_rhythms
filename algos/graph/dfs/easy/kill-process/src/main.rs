use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");

    let mut pid = vec![5, 2, 0, 1, 6, 3, 4];
    let mut ppid = vec![0, 5, 5, 5, 2, 2, 1];
    let kill = 5;
    println!("killed={:?}", kill_process(&mut pid, &mut ppid, kill));
}

type GRAPH = HashMap<i32, Vec<i32>>;
fn kill_process(pid: &mut Vec<i32>, ppid: &mut Vec<i32>, kill: i32) -> Vec<i32> {
    let mut graph: GRAPH = HashMap::new();

    for i in 0..pid.len() {
        let from = ppid[i];
        let to = pid[i];
        add_edge(&mut graph, from, to);
    }

    println!("graph={:?}", graph);

    reachability(&graph, kill)
}

fn add_edge(graph: &mut GRAPH, from: i32, to: i32) {
    if from == 0 {
        return;
    }
    graph.entry(from).or_insert(vec![]).push(to);
}

fn dfs(graph: &GRAPH, node: i32, visited: &mut HashSet<i32>) {
    visited.insert(node);

    if let Some(neighbors) = graph.get(&node) {
        for neighbor in neighbors {
            if visited.get(neighbor).is_none() {
                dfs(graph, *neighbor, visited);
            }
        }
    }
}

fn reachability(graph: &GRAPH, node: i32) -> Vec<i32> {
    let mut visited = HashSet::new();
    dfs(graph, node, &mut visited);
    visited.iter().map(|&x| x).collect()
}
