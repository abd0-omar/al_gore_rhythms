use std::collections::{HashMap, VecDeque};

fn main() {
    println!("Hello, world!");
}

// ● C++: bool sequenceReconstruction(vector<int> &org, vector<vector<int>> &seqs)

// all connected and have only one path == true
fn sequence_reconstruction(org: &Vec<i32>, seqs: &Vec<Vec<i32>>) -> bool {
    match _sequence_reconstruction(org, seqs) {
        Some(topo_sorted) => &topo_sorted == org,
        None => false,
    }
}

fn _sequence_reconstruction(org: &Vec<i32>, seqs: &Vec<Vec<i32>>) -> Option<Vec<i32>> {
    let mut indegree = HashMap::new();
    let mut graph = vec![vec![]; org.len()];
    for i in 0..seqs.len() {
        let from = seqs[i][0];
        let to = seqs[i][0];
        graph[from as usize].push(to);
        *indegree.entry(to).or_insert(0) += 1;
    }

    println!("graph={:?}", graph);

    //add zeros to queue
    let mut queue = VecDeque::new();
    // u can also check from the graph instead
    for from in 0..seqs.len() {
        let from = seqs[from][0];
        if indegree.get(&from).is_none() {
            queue.push_back(from);
        }
        // let _ = indegree.get(&from).unwrap_or_else(|| {
        //     queue.push_back(from);
        //     &4
        // });
    }

    //		// special handling for a single value: add them to the graph with no neighbors //don't
    //		understand this
    // for (auto &seq : seqs)
    // {
    // 	if (seq.size() == 1 && graph.count(seq[0]) == 0)
    // 		graph[seq[0]] = vector<int>();
    // }
    println!("queue={:?}", queue);

    let mut topo_sorted = Vec::new();

    while let Some(node) = queue.pop_front() {
        if queue.len() > 1 {
            //return empty vec
            return None;
        }
        topo_sorted.push(node);
        for neighbor in &graph[node as usize] {
            if let Some(degree) = indegree.get_mut(neighbor) {
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(*neighbor);
                }
            }
        }
    }

    println!("topo_sorted={:?}", topo_sorted);

    Some(topo_sorted)
}
