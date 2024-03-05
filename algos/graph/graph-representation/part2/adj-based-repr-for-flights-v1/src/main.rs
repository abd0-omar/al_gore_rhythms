use std::collections::{BTreeMap, BTreeSet};
use std::io;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    to: String,
    cost: i32,
}

// this trait is already implemented to sort them as needed by the derive macro
// , so no need to write them again
// impl Ord for Edge {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.to
//             .cmp(&other.to)
//             .then_with(|| self.cost.cmp(&other.cost))
//     }
// }

type Graph = BTreeMap<String, BTreeSet<Edge>>;

fn add_directed_edge(graph: &mut Graph, from: String, to: String, cost: i32) {
    graph
        .entry(from)
        .or_insert(BTreeSet::new())
        .insert(Edge { to, cost });
}

fn print_adjacency_matrix(graph: &Graph) {
    for (node, edges) in graph.iter() {
        println!("Flights from {}:", node);
        for edge in edges.iter() {
            println!("\tTo {} with cost {}", edge.to, edge.cost);
        }
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut line = line.split_whitespace();

    let _nodes: usize = line.next().unwrap().parse().unwrap();

    let edges: usize = line.next().unwrap().parse().unwrap();

    let mut graph: Graph = BTreeMap::new();

    for _ in 0..edges {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();

        let from: String = line.next().unwrap().into();

        let to: String = line.next().unwrap().into();

        let cost: i32 = line.next().unwrap().parse().unwrap();

        add_directed_edge(&mut graph, from.clone(), to.clone(), cost);
    }

    print_adjacency_matrix(&graph);
}
