use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

// Input: status = [1,0,1,0], candies = [7,5,4,100], keys = [[],[],[1],[]], containedBoxes = [[1,2],[3],[],[]], initialBoxes = [0]
// Output: 16
// Explanation: You will be initially given box 0. You will find 7 candies in it and boxes 1 and 2.
// Box 1 is closed and you do not have a key for it so you will open box 2. You will find 4 candies and a key to box 1 in box 2.
// In box 1, you will find 5 candies and box 3 but you will not find a key to box 3 so box 3 will remain closed.
// Total number of candies collected = 7 + 4 + 5 = 16 candy.

pub fn max_candies(
    mut status: Vec<i32>,
    candies: Vec<i32>,
    keys: Vec<Vec<i32>>,
    contained_boxes: Vec<Vec<i32>>,
    initial_boxes: Vec<i32>,
) -> i32 {
    let mut collected = 0;
    let mut visited = vec![false; status.len()];
    let mut q = VecDeque::new();

    add_keys_from_boxes(&mut status, &keys, &initial_boxes);
    open_boxes(
        &mut q,
        &mut visited,
        &mut status,
        &candies,
        &initial_boxes,
        &mut collected,
    );

    // box name in rust is taken
    // let box = 5; -> Syntax error
    while let Some(box_num) = q.pop_front() {
        add_keys_from_boxes(&mut status, &keys, &contained_boxes[box_num as usize]);
        open_boxes(
            &mut q,
            &mut visited,
            &mut status,
            &candies,
            &contained_boxes[box_num as usize],
            &mut collected,
        );
    }
    collected
}

fn add_keys_from_boxes(status: &mut Vec<i32>, keys: &Vec<Vec<i32>>, boxes: &Vec<i32>) {
    for &box_num in boxes {
        for &key in &keys[box_num as usize] {
            status[key as usize] = 1;
        }
    }
}

fn open_boxes(
    q: &mut VecDeque<i32>,
    visited: &mut Vec<bool>,
    status: &mut Vec<i32>,
    candies: &Vec<i32>,
    boxes: &Vec<i32>,
    collected: &mut i32,
) {
    for &box_num in boxes {
        if !visited[box_num as usize] && status[box_num as usize] == 1 {
            q.push_back(box_num);
            visited[box_num as usize] = true;
            *collected += candies[box_num as usize];
        }
    }
}
