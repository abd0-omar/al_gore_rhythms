fn main() {
    println!("Hello, world!");
    let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
    // Output: [[0,1,3],[0,2,3]]
    all_paths_source_target(graph);
}

pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut visited = vec![false; graph.len()];
    let mut rezult = vec![vec![]; 0];
    _all_paths_source_target(&graph, &mut visited, 0, vec![0], &mut rezult);
    println!("rezult={:?}", rezult);
    rezult
}

pub fn _all_paths_source_target(
    graph: &Vec<Vec<i32>>,
    visited: &mut Vec<bool>,
    curr_node: i32,
    mut curr_vec: Vec<i32>,
    rezult: &mut Vec<Vec<i32>>,
) -> () {
    println!("curr_node={:?}", curr_node);
    if curr_node == graph.len() as i32 - 1 {
        // curr_vec.insert(0, 0);
        rezult.push(curr_vec);
        return;
    }
    for &neighbor in graph[curr_node as usize].iter() {
        if !visited[neighbor as usize] {
            visited[neighbor as usize] = true;
            curr_vec.push(neighbor);
            _all_paths_source_target(graph, visited, neighbor, curr_vec.clone(), rezult);
            curr_vec.pop();
            visited[neighbor as usize] = false;
        }
    }
}
