use std::io;

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
        add_edge(&mut graph, from, to);
    }

    print_len_of_2(&graph);
}

fn add_edge(graph: &mut GRAPH, from: usize, to: usize) {
    graph[from].push(to);
}

fn print_len_of_2(graph: &GRAPH) {
    for (from, vec) in graph.iter().enumerate() {
        for &to in vec {
            for &next_to in &graph[to] {
                println!("{} {} {}", from, to, next_to);
            }
        }
    }
}
