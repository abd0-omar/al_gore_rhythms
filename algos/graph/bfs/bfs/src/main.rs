use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
    let arr = vec![4, 2, 3, 0, 3, 1, 2];
    let start = 5;
    println!("{}", can_reach(arr, start));
    // Example 2:
    //
    let arr = vec![4, 2, 3, 0, 3, 1, 2];
    let start = 0;
    println!("{}", can_reach(arr, start));
    //     Output: true
    // Explanation:
    // One possible way to reach at index 3 with value 0 is:
    // index 0 -> index 4 -> index 1 -> index 3
    //     Example 3:
    //
    let arr = vec![3, 0, 2, 1, 2];
    let start = 2;
    println!("{}", can_reach(arr, start));
    //     Output: false
    // Explanation: There is no way to reach at index 1 with value 0.
}

pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    let mut graph: Vec<Vec<i32>> = vec![vec![]; arr.len()];

    for (i, &a) in arr.iter().enumerate() {
        let from = i;
        let to1 = i as i32 + a;
        let to2 = i as i32 - a;
        if to1 > 0 && to1 < arr.len() as i32 {
            graph[from].push(to1 as i32);
        }
        if to2 > 0 && to2 < arr.len() as i32 {
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
                //if !visited q.push(), visited = true
            }
        }
    }

    false
}

// 5
// 6, 4
//
//  5 | 5 + 1, 5 - 1
// Example 1:
//
//               0 1 2 3 4 5 6
// Input: arr = [4,2,3,0,3,1,2], start = 5
//     Output: true
// Explanation:
// All possible ways to reach at index 3 with value 0 are:
// index 5 -> index 4 -> index 1 -> index 3
//     index 5 -> index 6 -> index 4 -> index 1 -> index 3

// Input: nums = [2,3,1,1,4]
//     Output: true
// Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
