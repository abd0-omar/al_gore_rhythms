fn main() {
    println!("Hello, world!");
    let vec = vec![0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];
    println!("{}", _lis(&vec, 0, None));
    println!("{}", lis(vec));
}

fn lis(vec: Vec<i32>) -> i32 {
    let mut max = 0;
    for i in 0..vec.len() {
        max = max.max(lis_helper(&vec, i));
    }
    max
}

fn lis_helper(vec: &Vec<i32>, idx: usize) -> i32 {
    if idx == vec.len() {
        return 0;
    }

    let mut max = 0;
    for i in idx + 1..vec.len() {
        if vec[i] > vec[idx] {
            max = max.max(lis_helper(vec, i));
        }
    }
    max += 1;

    max
}

fn _lis(vec: &Vec<i32>, idx: usize, prev: Option<usize>) -> i32 {
    if idx == vec.len() {
        return 0;
    }

    // leave
    let choice1 = _lis(vec, idx + 1, prev);

    // pick
    let mut choice2 = 0;
    match prev {
        Some(p) => {
            if vec[idx] > vec[p] {
                choice2 = 1 + _lis(vec, idx + 1, Some(idx));
            }
        }
        None => choice2 = 1 + _lis(vec, idx + 1, Some(idx)),
    }

    choice1.max(choice2)
}
