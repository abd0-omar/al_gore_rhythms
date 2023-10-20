use std::collections::BTreeMap;
use std::fmt;
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut line = line.split_whitespace();

    let nodes: usize = line.next().unwrap().parse().unwrap();

    let edges: usize = line.next().unwrap().parse().unwrap();

    let mut graph: GRAPH = vec![vec![]; nodes];
    let mut mapper: Mapper = Mapper::default();

    for _ in 0..edges {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();

        let from = line.next().unwrap();

        let to = line.next().unwrap();

        let cost: i32 = line.next().unwrap().parse().unwrap();

        let from_mapped = mapper.get_id(from);
        let to_mapped = mapper.get_id(to);

        add_edge(&mut graph, from_mapped, to_mapped, cost);
    }

    print(&graph, &mapper);
}

type GRAPH = Vec<Vec<Edge>>;

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    weight: i32,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.to, self.weight)
    }
}

// very cool idea/trick
#[derive(Default)]
struct Mapper {
    str_to_id: BTreeMap<String, usize>,
    id_to_str: BTreeMap<usize, String>,
}

impl Mapper {
    fn get_id(&mut self, str: &str) -> usize {
        match self.str_to_id.get(str) {
            Some(id) => *id,
            None => {
                let id = self.str_to_id.len();
                self.str_to_id.insert(str.to_string(), id);
                self.id_to_str.insert(id, str.to_string());
                id
            }
        }
    }

    fn get_str(&self, id: usize) -> String {
        self.id_to_str.get(&id).unwrap().into()
    }
}

fn add_edge(graph: &mut GRAPH, from: usize, to: usize, cost: i32) {
    graph[from].push(Edge { to, weight: cost });
}

fn print(graph: &GRAPH, mapper: &Mapper) {
    for (from, edges) in graph.iter().enumerate() {
        println!("Node {} has neighbors:", mapper.get_str(from));
        for edge in edges {
            println!("(to {} cost {})", mapper.get_str(edge.to), edge.weight);
        }
    }
}
