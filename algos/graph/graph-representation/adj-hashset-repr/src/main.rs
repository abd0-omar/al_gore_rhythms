use std::{collections::HashSet, io::stdin};

fn main() {
    println!("enter no. of nodes: ");
    let mut nodes_no = String::new();
    stdin().read_line(&mut nodes_no).unwrap();
    let nodes_no = nodes_no.trim().parse::<i32>().unwrap();

    println!("enter no. of edges: ");
    let mut edges_no = String::new();
    stdin().read_line(&mut edges_no).unwrap();
    let edges_no = edges_no.trim().parse::<i32>().unwrap();

    let mut graph: GRAPH = vec![HashSet::new(); nodes_no as usize];

    for _ in 0..edges_no {
        println!("enter edge from: ");
        let mut from = String::new();
        stdin().read_line(&mut from).unwrap();
        let from = from.trim().parse::<i32>().unwrap();

        println!("enter edge to: ");
        let mut to = String::new();
        stdin().read_line(&mut to).unwrap();
        let to = to.trim().parse::<i32>().unwrap();

        add_edge(&mut graph, from, to);
    }
    print(&graph)
}

type GRAPH = Vec<HashSet<i32>>;

fn add_edge(graph: &mut GRAPH, from: i32, to: i32) {
    graph[from as usize].insert(to);
}

fn print(graph: &GRAPH) {
    println!("{:?}", graph);
}
