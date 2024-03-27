fn main() {
    println!("Hello, world!");
}

// didn't build the graph on the fly, tho it's easy to do it now, but if it works don't touch it.
pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    use std::collections::VecDeque;
    let mut graph: Vec<Vec<i32>> = vec![vec![]; arr.len()];

    for (i, &a) in arr.iter().enumerate() {
        let from = i;
        let to1 = i as i32 + a;
        let to2 = i as i32 - a;
        if to1 >= 0 && to1 < arr.len() as i32 {
            graph[from].push(to1 as i32);
        }
        if to2 >= 0 && to2 < arr.len() as i32 {
            graph[from].push(to2 as i32);
        }
    }

    println!("graph={:?}", graph);

    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut visited = vec![false; arr.len()];
    // visited[start as usize] = true;
    let size = queue.len();

    while !queue.is_empty() {
        for _ in 0..size {
            // let idx = queue.pop_front().unwrap() as usize; //5
            // if visited[idx] {
            //     continue;
            // }
            // if  arr[idx] == 0 {
            //     return true;
            // }
            // visited[idx] = true;
            // queue.push_back(value)
            // unimplemented!();
            let curr = queue.pop_front().unwrap();
            println!("curr={:?}", curr);
            if arr[curr as usize] == 0 {
                return true;
            }
            if visited[curr as usize] {
                continue;
            }
            visited[curr as usize] = true;
            for &neighbor in &graph[curr as usize] {
                queue.push_back(neighbor);
            }
        }
    }

    false
}
