type GRAPH = Vec<Vec<i32>>;

fn main() {
    println!("Hello, world!");

    let mut pid = vec![5, 2, 0, 1, 6, 3, 4];
    let mut ppid = vec![0, 5, 5, 5, 2, 2, 1];
    let kill = 2;
    println!("{:?}", kill_process(&mut pid, &mut ppid, kill));
}

fn kill_process(pid: &mut Vec<i32>, ppid: &mut Vec<i32>, kill: i32) -> Vec<i32> {
    let mut graph: GRAPH = vec![vec![]; pid.len()];

    for i in 0..pid.len() {
        let from = ppid[i];
        let to = pid[i];
        add_edge(&mut graph, from, to);
    }

    println!("{:?}", graph);

    let reach_vec = reachability(&graph, &pid);

    reach_vec[kill as usize].clone()
}

fn add_edge(graph: &mut GRAPH, from: i32, to: i32) {
    if from == 0 {
        return;
    }
    graph[from as usize].push(to);
}

fn dfs(graph: &GRAPH, node: i32, visited: &mut Vec<bool>, reach: &mut Vec<i32>) {
    // add a parameter that updates the vec of reachability
    visited[node as usize] = true;

    for &neighbor in &graph[node as usize] {
        // add to the reachability vec
        if visited[neighbor as usize] != true {
            println!("neighbor={:?}", neighbor);
            reach.push(neighbor);
            dfs(graph, neighbor, visited, reach);
        }
    }
}

fn reachability(graph: &GRAPH, pid: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut reach: Vec<Vec<i32>> = vec![vec![]; pid.len()];
    for &p in pid {
        println!("p={:?}", p);
        let mut visited = vec![false; graph.len()];

        // another vec = reach[i]

        let mut another = vec![];

        dfs(graph, p, &mut visited, &mut another);

        println!("another={:?}", another);

        reach[p as usize].push(p);
        reach[p as usize].extend_from_slice(&another);
        println!("reach={:?}", reach);

        // reach[i].push(another vec);
    }

    reach
}

//vector<int> killProcess(vector<int> &pid, vector<int> &ppid, int kill)
