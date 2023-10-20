use std::io;

type Graph = Vec<Vec<Vec<i32>>>;

fn print_adjacency_matrix(graph: &Graph) {
    let nodes = graph.len();
    for from in 0..nodes {
        for to in 0..nodes {
            for &weight in &graph[from][to] {
                println!("From {} to {} the cost is {}", from, to, weight);
            }
        }
    }
}

fn add_directed_edge(graph: &mut Graph, from: usize, to: usize, cost: i32) {
    graph[from][to].push(cost);
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let mut parts = input.split_whitespace();
    let nodes: usize = parts
        .next()
        .expect("Missing node count")
        .parse()
        .expect("Invalid node count");
    let edges: usize = parts
        .next()
        .expect("Missing edge count")
        .parse()
        .expect("Invalid edge count");

    let mut graph: Graph = vec![vec![vec![]; nodes]; nodes];

    for _ in 0..edges {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let mut parts = input.split_whitespace();
        let from: usize = parts
            .next()
            .expect("Missing from")
            .parse()
            .expect("Invalid from");
        let to: usize = parts
            .next()
            .expect("Missing to")
            .parse()
            .expect("Invalid to");
        let cost: i32 = parts
            .next()
            .expect("Missing cost")
            .parse()
            .expect("Invalid cost");
        add_directed_edge(&mut graph, from, to, cost);
    }

    print_adjacency_matrix(&graph);
}
