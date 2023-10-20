use core::cmp::Ordering;
use std::{fmt, io::stdin};

type GRAPH = Vec<Edge>;
//TODO: make it implement comparable and display

// ● typedef vector<edge> GRAPH;
fn main() {
    println!("Hello, world!");

    println!("enter no. of nodes: ");
    let mut nodes_no = String::new();
    stdin().read_line(&mut nodes_no).unwrap();
    let nodes_no = nodes_no.trim().parse::<i32>().unwrap();

    println!("enter no. of edges: ");
    let mut edges_no = String::new();
    stdin().read_line(&mut edges_no).unwrap();
    let edges_no = edges_no.trim().parse::<i32>().unwrap();

    let mut graph: GRAPH = Vec::with_capacity(nodes_no as usize);

    for _ in 0..edges_no {
        println!("enter edge from: ");
        let mut from = String::new();
        stdin().read_line(&mut from).unwrap();
        let from = from.trim().parse::<i32>().unwrap();

        println!("enter edge to: ");
        let mut to = String::new();
        stdin().read_line(&mut to).unwrap();
        let to = to.trim().parse::<i32>().unwrap();

        println!("enter edge cost: ");
        let mut cost = String::new();
        stdin().read_line(&mut cost).unwrap();
        let cost = cost.trim().parse::<i32>().unwrap();

        add_edge(&mut graph, from, to, cost);
    }
    // graph.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());
    graph.sort();
    print_adjaceny_matrix(&graph);
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Edge {
    from: i32,
    to: i32,
    cost: i32,
}

//for printing the edge not GRAPH
impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"   {}
{} --> {}"#,
            self.cost, self.from, self.to
        )
    }
}

//for sorting by cost in stead of sort_by()
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

fn add_edge(graph: &mut GRAPH, from: i32, to: i32, cost: i32) {
    let edge = Edge { from, to, cost };
    graph.push(edge);
}

fn print_adjaceny_matrix(graph: &GRAPH) {
    for g in graph {
        println!("{}", g);
    }
}
