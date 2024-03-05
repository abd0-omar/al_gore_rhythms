use std::collections::HashMap;

fn main() {
    let adjacent_pairs = vec![vec![2, 1], vec![3, 4], vec![3, 2]];
    // Output: [1,2,3,4]
    let adjacent_pairs = vec![vec![4, -2], vec![1, 4], vec![-3, 1]];
    // Output: [-2,4,1,-3]
    println!("{:?}", restore_array(adjacent_pairs));
}

pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    // let mut start = vec![-1; adjacent_pairs.len()];
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for pair in adjacent_pairs.iter() {
        let from = pair[0];
        let to = pair[1];
        // graph[from].push(to);
        // graph[to].push(from);
        graph.entry(from).or_insert(vec![]).push(to);
        graph.entry(to).or_insert(vec![]).push(from);
    }
    println!("DEBUGPRINT[1]: main.rs:16: graph={:?}", graph);
    let mut start: HashMap<i32, i32> = HashMap::new();

    let mut chain = Vec::new();
    let mut time = 1;
    for (&g, v) in &graph {
        if v.len() > 1 || start.get(&g).is_some() {
            continue;
        }
        dfs(&graph, g, &mut start, &mut chain, &mut time);
    }
    // for pair in adjacent_pairs {
    //     for p in pair {
    //         if start.get(&p).is_some(){
    //             continue;
    //         }
    //         dfs(&graph, p, &mut start, &mut chain, &mut time);
    //     }
    // }

    println!("DEBUGPRINT[2]: main.rs:19: start={:?}", start);
    println!("DEBUGPRINT[3]: main.rs:21: chain={:?}", chain);

    // let mut f = start.clone().into_keys().collect::<Vec<i32>>();
    // println!("DEBUGPRINT[3]: main.rs:37: f={:?}", f);
    // let start_values: Vec<i32> = start.clone().into_values().collect();
    // println!("DEBUGPRINT[5]: main.rs:39: start_values={:?}", start_values);
    // // sort f based on start values
    // f.sort_unstable_by(|&a, &b| start[&a].cmp(&start[&b]));
    // println!("DEBUGPRINT[4]: main.rs:41: f={:?}", f);

    chain
}

fn dfs(
    graph: &HashMap<i32, Vec<i32>>,
    current: i32,
    start: &mut HashMap<i32, i32>,
    chain: &mut Vec<i32>,
    time: &mut i32,
) {
    // start[&current] += 1;
    *start.entry(current).or_insert(0) += *time;
    *time += 1;

    chain.push(current);
    for neighbor in &graph[&current] {
        // if start[neighbor] == -1 {
        if start.get(neighbor).is_none() {
            dfs(graph, *neighbor, start, chain, time);
        } else {
            // if it is backward don't put in the cahin
            if start[&current] > start[neighbor] {
                // chain.push(*neighbor);
                println!("hello");
            }
        }
    }
}
