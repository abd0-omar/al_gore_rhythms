use std::{collections::HashSet, io::stdin};

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap();

    let mut graph: GRAPH;
    // add_edge(&mut graph, from, to);
}

type GRAPH = Vec<HashSet<i32>>;

fn add_edge(graph: &mut GRAPH, from: i32, to: i32) {
    graph[from as usize].insert(to);
}

fn print(graph: &GRAPH) {
    println!("{:?}", graph);
}
