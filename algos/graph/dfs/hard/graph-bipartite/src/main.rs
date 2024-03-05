fn main() {
    let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
    // Output: false
    // Explanation: There is no way to partition the nodes into two independent sets such that every edge connects a node in one and a node in the other.
    // Example 2:

    println!("{}", is_bipartite(graph));

    let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
    println!("{}", is_bipartite(graph));
    // Output: true
}

pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let mut validation = true;
    let mut color_vec = vec![0; graph.len()];

    for i in 0..graph.len() {
        if color_vec[i] == 0 {
            dfs(&graph, i as i32, &mut validation, 1, &mut color_vec);
            if !validation {
                return false;
            }
        }
    }
    validation
}

fn dfs(
    graph: &Vec<Vec<i32>>,
    node: i32,
    validation: &mut bool,
    color: i32,
    color_vec: &mut Vec<i32>,
) {
    if color_vec[node as usize] == 0 {
        color_vec[node as usize] = color;
    } else {
        if color_vec[node as usize] != color {
            *validation = false;
        }
        return;
    }

    for &neighbor in &graph[node as usize] {
        dfs(graph, neighbor, validation, 3 - color, color_vec);
    }
}
