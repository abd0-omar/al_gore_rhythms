use std::io;

type GRAPH = Vec<Vec<usize>>;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut line = line.split_whitespace();

    let r: usize = line.next().unwrap().parse().unwrap();
    let c: usize = line.next().unwrap().parse().unwrap();

    let mut graph: GRAPH = vec![vec![]; r * c];

    // add_edge(&mut graph, r, c)
    add_edge(&mut graph, r, c);
    println!("{:?}", graph);
    print(&graph);
}

// I was better of using not flatten idices to check if valid
fn add_edge(graph: &mut GRAPH, r: usize, c: usize) {
    for row_idx in 0..r {
        for col_idx in 0..c {
            let idx = row_idx * c + col_idx;
            if idx % c != 0 {
                // idx - 1 -> 4 % 4 != 0
                graph[idx].push(idx - 1);
            }
            if (idx + 1) % c != 0 {
                // idx + 1 -> 8 % 4 != 0
                graph[idx].push(idx + 1);
            }
            if idx as i32 - c as i32 >= 0 {
                graph[idx].push(idx - c);
            }

            if idx + c > (r * c) - 1 {
                continue;
            }
            graph[idx].push(idx + c);
        }
    }
}

fn print(graph: &GRAPH) {
    for (node_idx, node_vec) in graph.iter().enumerate() {
        print!("node {} has neighbors: ", node_idx);
        for neighbor in node_vec {
            print!("{} ", neighbor);
        }
        println!();
    }
}
