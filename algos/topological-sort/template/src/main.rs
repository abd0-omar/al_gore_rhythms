use std::{collections::VecDeque, fs};

fn main() {
    println!("Hello, world!");
    let read_file = fs::read_to_string("data.txt").expect("can't read file");
    let mut input = read_file.split_whitespace();
    let mut cases = input.next().unwrap().parse::<usize>().unwrap();
    while cases > 0 {
        let nodes = input.next().unwrap().parse::<usize>().unwrap();
        let edges = input.next().unwrap().parse::<usize>().unwrap();

        let mut graph = vec![vec![]; nodes];

        for _ in 0..edges {
            let from = input.next().unwrap().parse::<usize>().unwrap();
            let to = input.next().unwrap().parse::<i32>().unwrap();
            graph[from].push(to);
        }

        println!("{:?}", topo_sort(graph));

        cases -= 1;
    }

    // for input in read_file.split_whitespace() {
    //
    // }
}

fn topo_sort(adj_list: Vec<Vec<i32>>) -> Option<Vec<i32>> {
    // compute indegree for all nodes
    let mut indegree = vec![0; adj_list.len()];
    for node in 0..adj_list.len() {
        for &neighbor in &adj_list[node] {
            indegree[neighbor as usize] += 1;
        }
    }

    // println!("indegree={:?}", indegree);

    // add all zeros to queue

    let mut ready = VecDeque::new();

    for (i, &node_degree) in indegree.iter().enumerate() {
        if node_degree == 0 {
            ready.push_back(i as i32);
        }
    }

    // println!("ready={:?}", ready);

    // try one-liner

    // put all of them sorted in a vec relative to their indegree
    let mut ordered = Vec::new();
    while let Some(node) = ready.pop_front() {
        ordered.push(node);
        // decrease it's neighbor degree and check if it is zero to put it in queue
        for &neighbor in &adj_list[node as usize] {
            //check degree
            indegree[neighbor as usize] -= 1;
            if indegree[neighbor as usize] == 0 {
                ready.push_back(neighbor);
            }
        }
    }

    // check if the nodes in ordred == adj_list (if not then has cycle)
    if ordered.len() != adj_list.len() {
        return None;
    }
    Some(ordered)
}
