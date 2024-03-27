fn main() {
    println!("Hello, world!");
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let num_courses = num_courses as usize;
    let mut graph = vec![vec![]; num_courses];
    let mut indegree = vec![0; num_courses];

    // build the graph from the given edges and fin out the nodes' indegree
    for edge in prerequisites.into_iter() {
        graph[edge[0] as usize].push(edge[1] as usize);
        indegree[edge[1] as usize] += 1;
    }

    // find all nodes without incoming edges
    let mut safe = vec![];
    for (node, &degree) in indegree.iter().enumerate() {
        if degree == 0 {
            safe.push(node);
        }
    }

    // The nodes with indegree of 0 come before the other nodes.
    while let Some(node) = safe.pop() {
        for &next in graph[node].iter() {
            // reduce the indegree of each neighbour
            // if after that operation it has an indegree of 0, then we can push it on the queue/stack
            indegree[next] -= 1;
            if indegree[next] == 0 {
                safe.push(next);
            }
        }
    }

    // If there are any nodes left with indegree != 0, then there is a
    // cycle and the graph cannot be topologically sorted

    if indegree.len() != graph.len() {
        return false;
    }

    true
}

pub fn can_finish_dfs(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let num_courses = num_courses as usize;
    // build the graph from the given edges
    let mut graph = vec![vec![]; num_courses];
    for edge in prerequisites.into_iter() {
        graph[edge[0] as usize].push(edge[1] as usize);
    }

    // because the graph may not be connected we have to go over all nodes
    let mut visited = vec![Marker::Unvisited; num_courses];
    for node in 0..num_courses {
        // If we have already processed this node, then skip it in order to
        // save time - we already know that it does not contain a cycle
        if visited[node] == Marker::Unvisited {
            let mut is_cycle = false;
            has_cycle_dfs(&graph, &mut visited, node, &mut is_cycle);
            if is_cycle {
                return false;
            }
        }
    }

    true
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Marker {
    Unvisited,
    Undecided,
    Processed,
}

fn has_cycle_dfs(graph: &[Vec<usize>], visited: &mut [Marker], node: usize, is_cycle: &mut bool) {
    match visited[node] {
        // We are in a cycle
        Marker::Undecided => {
            *is_cycle = true;
            return;
        }
        // We have already processed that subtree and it does not contain a cycle
        Marker::Processed => return,
        // Check if the current subtree is part of a cycle. We set this state
        // only on unvisited nodes. If we visit them again - then we are in a
        // cycle, otherwise we mark them as "Processed"
        Marker::Unvisited => visited[node] = Marker::Undecided,
    }

    for &next in graph[node].iter() {
        has_cycle_dfs(graph, visited, next, is_cycle);
        // if *is_cycle {
        //     return;
        // }
    }

    visited[node] = Marker::Processed;
}
