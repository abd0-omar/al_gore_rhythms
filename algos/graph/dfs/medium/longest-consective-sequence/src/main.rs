use std::collections::{HashMap, HashSet};

type Graph = HashMap<i32, Vec<i32>>;

fn add_undirected_edge(graph: &mut Graph, from: i32, to: i32) {
    graph.entry(from).or_insert_with(Vec::new).push(to);
    graph.entry(to).or_insert_with(Vec::new).push(from);
}

fn dfs(graph: &Graph, node: i32, visited: &mut HashSet<i32>) -> i32 {
    visited.insert(node);

    let mut chain_size = 1;

    if let Some(neighbors) = graph.get(&node) {
        for &neighbour in neighbors {
            if !visited.contains(&neighbour) {
                chain_size += dfs(graph, neighbour, visited);
            }
        }
    }

    chain_size
}

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let nums_set: HashSet<i32> = nums.into_iter().collect();

    if nums_set.is_empty() {
        return 0;
    }

    let mut graph: Graph = HashMap::new();

    for &val in &nums_set {
        if nums_set.contains(&(val + 1)) {
            println!("DEBUGPRINT[2]: main.rs:35: val={:?}", val);
            add_undirected_edge(&mut graph, val, val + 1);
        }
    }

    let mut max_chain_size = 1;
    let mut visited: HashSet<i32> = HashSet::new();

    for (&node, neighbors) in &graph {
        if !visited.contains(&node) && neighbors.len() == 1 {
            // the len for the first element of
            // the rooted tree
            let chain_size = dfs(&graph, node, &mut visited);
            max_chain_size = max_chain_size.max(chain_size);
        }
    }

    max_chain_size
}

fn main() {
    let nums = vec![100, 200, 5, 4, 3, 300, 2, 1];
    let result = longest_consecutive(nums);
    println!("Longest consecutive chain size: {}", result);
}
