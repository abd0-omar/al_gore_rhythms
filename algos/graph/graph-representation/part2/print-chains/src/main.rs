use std::io;

fn add_directed_edge(graph: &mut GRAPH, from: usize, to: usize) {
    graph[from].push(to);
}

// dfs
fn print_chain(graph: &GRAPH, from: Option<usize>) {
    if from.is_none() {
        return;
    }
    let from = from.unwrap();
    print!("{} ", from);

    // if let Some(&next) = graph.get(from).and_then(|neighbors| neighbors.first()) {
    if let Some(next) = graph.get(from) {
        let next = next.first().cloned();
        print_chain(graph, next);
    }
}

type GRAPH = Vec<Vec<usize>>;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut line = line.split_whitespace();
    let nodes: usize = line.next().unwrap().parse().unwrap();
    let edges: usize = line.next().unwrap().parse().unwrap();

    let mut graph: GRAPH = vec![Vec::new(); nodes];

    for _ in 0..edges {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();
        let from: usize = line.next().unwrap().parse().unwrap();
        let to: usize = line.next().unwrap().parse().unwrap();
        add_directed_edge(&mut graph, from, to);
    }

    let mut queries = String::new();
    io::stdin().read_line(&mut queries).unwrap();
    let queries: usize = queries.trim().parse().unwrap();

    for _ in 0..queries {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let node: usize = line.trim().parse().unwrap();
        print_chain(&graph, Some(node));
        println!();
    }
}
