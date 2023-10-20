fn main() {
    println!("Hello, world!");
    let s = "dcab".to_string();
    let pairs = vec![vec![0, 3], vec![1, 2], vec![0, 2]];
    // Output: "abcd"
    println!("{}", smallest_string_with_swaps(s, pairs));
    // let s = "dcab".to_string();
    // let pairs = vec![vec![0, 3], vec![1, 2]];
    // // Output: "bacd"
    // println!("{}", smallest_string_with_swaps(s, pairs));
}

pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
    let mut s: Vec<char> = s.chars().collect();
    let mut graph: Vec<Vec<usize>> = vec![vec![]; s.len()];

    for i in 0..pairs.len() {
        graph[pairs[i][0] as usize].push(pairs[i][1] as usize);
        graph[pairs[i][1] as usize].push(pairs[i][0] as usize);
    }

    println!("DEBUGPRINT[9]: main.rs:19: graph={:?}", graph);
    let mut components: Vec<Vec<usize>> = Vec::new();
    let mut visited = vec![false; s.len()];

    for i in 0..s.len() {
        if !visited[i] {
            let mut chain = Vec::new();
            dfs(&graph, i, &mut s, &mut chain, &mut visited);
            println!("DEBUGPRINT[8]: main.rs:29: chain={:?}", chain);
            components.push(chain);
        }
    }
    println!("DEBUGPRINT[7]: main.rs:29: components={:?}", components);

    // Sort each component and use the sorted values
    for mut component in components {
        component.sort();
        println!("DEBUGPRINT[10]: main.rs:38: component={:?}", component);
        let mut chain_string: Vec<char> = component.iter().map(|&c| s[c]).collect();
        chain_string.sort();

        for (idx, &c) in component.iter().enumerate() {
            s[c] = chain_string[idx];
        }
    }

    s.iter().collect()
}

fn dfs(
    graph: &Vec<Vec<usize>>,
    current: usize,
    s: &mut Vec<char>,
    chain: &mut Vec<usize>,
    visited: &mut Vec<bool>,
) {
    visited[current] = true;
    chain.push(current);
    for &neighbor in &graph[current] {
        if !visited[neighbor] {
            dfs(graph, neighbor, s, chain, visited);
        }
    }
}
