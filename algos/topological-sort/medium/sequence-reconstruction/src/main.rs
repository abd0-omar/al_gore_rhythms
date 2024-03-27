use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let org = vec![1, 2, 3];
    let seqs = vec![vec![1, 2], vec![2, 3], vec![3, 1]];
    println!("{}", sequence_reconstruction(&org, &seqs));

    let org = vec![1, 2, 3];
    let seqs = vec![vec![1, 2], vec![1, 3]];
    println!("{}", sequence_reconstruction(&org, &seqs));

    println!("Hello, world!");

    let org = vec![4, 1, 5, 2, 6, 3];
    let seqs = vec![vec![5, 2, 6, 3], vec![4, 1, 5, 2]];
    println!("{}", sequence_reconstruction(&org, &seqs));

    let org = vec![1, 2, 3];
    let seqs = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    println!("{}", sequence_reconstruction(&org, &seqs));

    let org = vec![1, 2, 3];
    let seqs = vec![vec![1, 2]];
    println!("{}", sequence_reconstruction(&org, &seqs));
}

// ● C++: bool sequenceReconstruction(vector<int> &org, vector<vector<int>> &seqs)

// all connected and have only one path == true

fn sequence_reconstruction(org: &Vec<i32>, seqs: &Vec<Vec<i32>>) -> bool {
    let mut indegree = HashMap::new();
    let mut graph = HashMap::new();

    for seq in seqs {
        for j in 0..seq.len() - 1 {
            let from = seq[j];
            let to = seq[j + 1];
            graph.entry(from).or_insert(HashSet::new()).insert(to);
            continue;
        }
        if seq.len() == 1 {
            let from = seq[0];
            graph.insert(from, HashSet::new());
        }
    }
    for (k, v) in graph.iter() {
        for one_v in v {
            let from = k;
            let to = one_v;
            if indegree.get(&from).is_none() {
                indegree.insert(from, 0);
            }
            *indegree.entry(to).or_insert(0) += 1;
        }
    }

    println!("graph={:?}", graph);
    println!("indegree={:?}", indegree);

    //add zeros to queue
    let mut queue = VecDeque::new();
    let mut topo_sorted = Vec::new();
    for k in graph.keys() {
        if indegree[&k] == 0 {
            queue.push_back(k);
            topo_sorted.push(k);
        }
    }

    while !queue.is_empty() {
        println!("queue={:?}", queue);
        let size = queue.len();
        if size > 1 {
            return false;
        }
        let curr = queue.pop_front().unwrap();

        if let Some(neighbor_hash_set) = graph.get(curr) {
            for neighbor in neighbor_hash_set {
                if let Some(in_deg) = indegree.get_mut(neighbor) {
                    *in_deg -= 1;
                }
                if indegree[neighbor] == 0 {
                    topo_sorted.push(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    if topo_sorted.len() != indegree.len() {
        println!("there is a cycle");
        return false;
    }

    &topo_sorted.iter().cloned().cloned().collect::<Vec<_>>() == org
}
