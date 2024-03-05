use std::collections::{HashSet, VecDeque};

fn main() {
    // Example 1:
    let deadends = vec![
        "0201".to_string(),
        "0101".to_string(),
        "0102".to_string(),
        "1212".to_string(),
        "2002".to_string(),
    ];
    let target = "0202".to_string();
    println!("{}", open_lock(deadends, target)); // Output: 6

    // Example 2:
    let deadends = vec!["8888".to_string()];
    let target = "0009".to_string();
    println!("{}", open_lock(deadends, target)); // Output: 1

    // Example 3:
    let deadends = vec![
        "8887".to_string(),
        "8889".to_string(),
        "8878".to_string(),
        "8898".to_string(),
        "8788".to_string(),
        "8988".to_string(),
        "7888".to_string(),
        "9888".to_string(),
    ];
    let target = "8888".to_string();
    println!("{}", open_lock(deadends, target)); // Output: -1
}

fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let mut visited = HashSet::new();
    // for d in &deadends {
    //     visited.insert(d.clone());
    // }
    visited.extend(deadends.iter().cloned());

    let mut q = VecDeque::new();
    q.push_back("0000".to_string());
    visited.insert("0000".to_string());
    let mut lvl = 0;

    while !q.is_empty() {
        lvl += 1;
        let size = q.len();
        for _ in 0..size {
            let pop = q.pop_front().unwrap();
            for i in 0..4 {
                // let neighbors = get_neighbors(&pop, i);
                for neighbor in get_neighbors(&pop, i) {
                    if neighbor == target {
                        return lvl;
                    }

                    if visited.insert(neighbor.clone()) {
                        q.push_back(neighbor);
                    }
                }
            }
        }
    }

    -1
}

fn next(c: char) -> char {
    if c == '9' {
        '0'
    } else {
        char::from_digit(c.to_digit(10).unwrap() + 1, 10).unwrap()
    }
}

fn prev(c: char) -> char {
    if c == '0' {
        '9'
    } else {
        char::from_digit(c.to_digit(10).unwrap() - 1, 10).unwrap()
    }
}

fn get_neighbors(pop: &str, i: usize) -> Vec<String> {
    let mut neighbors = Vec::new();
    let mut lock_chars: Vec<char> = pop.chars().collect();

    lock_chars[i] = next(lock_chars[i]);
    neighbors.push(lock_chars.iter().collect::<String>());

    lock_chars[i] = prev(prev(lock_chars[i]));
    neighbors.push(lock_chars.iter().collect::<String>());

    // lock_chars[i] = next(lock_chars[i]);
    neighbors
}
