use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    println!("Hello, world!");
}

fn lexio(graph: Vec<Vec<i32>>) {
    let mut q = BinaryHeap::new();
    let mut indegree = vec![0; graph.len()];
    //compute indegree
    for i in 0..graph.len() {
        for j in 0..graph[i].len() {
            let to = graph[i][j] as usize;
            indegree[to] += 1;
        }
    }

    // push zeros
    for i in 0..indegree.len() {
        if indegree[i] == 0 {
            q.push(-(i as i32));
        }
    }

    let mut ordered = Vec::new();

    while let Some(node) = q.pop() {
        ordered.push(Reverse(node));

        for &neighbor in &graph[node as usize] {
            indegree[neighbor as usize] -= 1;

            if indegree[neighbor as usize] == 0 {
                q.push(-neighbor)
            }
        }
    }

    if ordered.len() != graph.len() {
        println!("cycle detected")
    }

    println!("ordered={:?}", ordered);
}
