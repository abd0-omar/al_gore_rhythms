fn main() {
    // println!("Hello, world!");
    let n = 1;
    let edges = vec![];
    // Output: [0,1]
    println!("{:?}", find_min_height_trees(n, edges));
    //
    // let n = 6;
    // let edges = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];
    // // Output: [3,4]
    // println!("{:?}", find_min_height_trees(n, edges));
    //
    // let n = 4;
    // let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3]];
    // // Output: [3,4]
    // println!("{:?}", find_min_height_trees(n, edges));
}

pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph = vec![vec![]; n as usize];

    for edge in edges {
        let from = edge[0];
        let to = edge[1];
        graph[from as usize].push(to);
        graph[to as usize].push(from);
    }

    println!("graph={:?}", graph);
    let mut in_degree = vec![None; n as usize];

    for neighbors_vec in graph.iter() {
        for &neighbor in neighbors_vec.iter() {
            in_degree[neighbor as usize] = Some(match in_degree[neighbor as usize] {
                Some(num) => num + 1,
                None => 0,
            });
        }
    }

    let mut lowest_max = 0;
    let mut rezult = vec![];
    let mut deg_sorted = in_degree.clone();
    deg_sorted.sort_unstable();
    for &deg in deg_sorted.iter() {
        if let Some(d) = deg {
            if d > lowest_max {
                lowest_max = d;
                break;
            }
        }
    }

    for (i, &deg) in in_degree.iter().enumerate() {
        if let Some(d) = deg {
            if d >= lowest_max {
                rezult.push(i as i32);
            }
        }
    }
    println!("lowest_max={:?}", lowest_max);

    println!("in_degree={:?}", in_degree);

    if rezult.is_empty() {
        vec![0]
    } else {
        rezult
    }
}
