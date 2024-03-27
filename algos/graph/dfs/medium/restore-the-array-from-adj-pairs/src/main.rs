use std::collections::{HashMap, HashSet};

fn main() {
    let adjacent_pairs = vec![vec![2, 1], vec![3, 4], vec![3, 2]];
    println!("{:?}", restore_array(adjacent_pairs));

    let adjacent_pairs2 = vec![vec![4, -2], vec![1, 4], vec![-3, 1]];
    println!("{:?}", restore_array(adjacent_pairs2));
}

pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    let mut adjacency_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for pair in &adjacent_pairs {
        adjacency_map.entry(pair[0]).or_insert(vec![]).push(pair[1]);
        adjacency_map.entry(pair[1]).or_insert(vec![]).push(pair[0]);
    }

    let mut result = Vec::new();
    // the start or the end has one neighbor, so we should start from there
    let start = *adjacency_map
        .keys()
        .find(|&key| adjacency_map[key].len() == 1)
        .unwrap();
    let mut visited = HashSet::new();

    dfs(start, &adjacency_map, &mut visited, &mut result);

    result
}

pub fn dfs(
    current: i32,
    adjacency_map: &HashMap<i32, Vec<i32>>,
    visited: &mut HashSet<i32>,
    result: &mut Vec<i32>,
) {
    visited.insert(current);
    result.push(current);

    if let Some(neighbors) = adjacency_map.get(&current) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                dfs(neighbor, adjacency_map, visited, result);
            }
        }
    }
}
