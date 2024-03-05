use std::{
    cmp::min,
    collections::{HashSet, VecDeque},
};

fn main() {
    println!("Hello, world!");
}

// ● jug1Capacity = 1, jug2Capacity = 2, targetCapacity = 3
// ○ True ⇒ Clearly adding them 1+2 ⇒ 3
// ● jug1Capacity = 10, jug2Capacity = 20, targetCapacity = 100
// ○ False ⇒ their max is 10+20
// ● jug1Capacity = 3, jug2Capacity = 5, targetCapacity = 6
// ○ Initially ⇒ (0, 0) ⇒ fill2 ⇒ (0, 5) pour 2 in 1 ⇒ (3, 2) ⇒ empty 2 ⇒ (3, 0) ⇒ pour 1 to 2 ⇒ (0,
// 3) ⇒ fill1 ⇒ (3, 3) which sum to 6
// ● Bouns: Modify your code to print the shortest operations sequence

//                (0, 0)
//           (3, 0)     (0, 5)
// (3, 5)  (0, 3)           (3, 5)  (3, 2)
//                              u can empty it too

pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
    if target_capacity == 0 {
        return true;
    }
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    let mut lvl = 0;

    while !q.is_empty() {
        lvl += 1;
        let size = q.len();
        for _ in 0..size {
            let cur = q.pop_front().unwrap();
            for neighbor in get_neighbors(&cur, jug1_capacity, jug2_capacity) {
                if neighbor.0 + neighbor.1 == target_capacity {
                    return true;
                }
                if visited.insert(neighbor) {
                    // sort them
                    q.push_back(neighbor);
                }
            }
        }
    }

    false
}

fn get_neighbors(cur: &(i32, i32), jug1: i32, jug2: i32) -> Vec<(i32, i32)> {
    let mut v = Vec::new();

    // too much simple math

    // (0, 5) -> (3, 2)
    let new_jug = (cur.0 + jug1, cur.1 - jug1);
    v.push(new_jug);
    //  (5, 0) -> (2, 3)
    let new_jug = (cur.1 + jug2, cur.0 - jug2);
    v.push(new_jug);
    //empty it
    let new_jug = (cur.0, 0);
    v.push(new_jug);
    let new_jug = (0, cur.0);
    v.push(new_jug);

    // fill
    let new_jug = (jug1, cur.1);
    v.push(new_jug);
    let new_jug = (cur.0, jug2);
    v.push(new_jug);

    // pour 1 to 2
    // pour all of it or pour some
    let m1 = min(cur.0, jug2 - cur.1);
    let m2 = min(cur.1, jug1 - cur.0);
    let new_jug = (cur.0 - m1, cur.1 + m1);
    v.push(new_jug);
    let new_jug = (cur.0 + m2, cur.1 - m2);
    v.push(new_jug);

    println!("v={:?}", v);
    v
}
