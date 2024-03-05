use std::usize;

// fn main() {
//     println!("hello, world!");
//     let w = vec![10, 4, 20, 5, 7];
//     let v = vec![10, 15, 3, 1, 4];
//     // println!("{}", knapsack(0, 0, 0, 12, &v, &w));
//     knapsack(&w, &v, 12, 0, w.len(), 0);
// }

// fn knapsack(v_sum: i32, w_sum: i32, i: usize, limit: i32, v: &vec<i32>, w: &vec<i32>) -> i32 {
//     if w_sum > limit || i >= 5 {
//         return -1;
//     }
//
//     // let mut w_sumz = w_sum + w[i];
//     // calculate for 1 element
//     // 4 + 10 -> 14
//     let mut w_sumz = w[i] + w_sum;
//     if w[i] + w_sum > limit {
//         w_sumz -= w[i];
//         knapsack(v_sum, w_sum, i + 1, limit, v, w);
//     }
//     println!("w_sumz={:#?}", w_sumz);
//     // 15 + 10 -> 25
//     let v_sumz = v[i] + v_sum;
//     println!("v_sumz={:#?}", v_sumz);
//
//     let mut max_sum = v_sumz;
//     if w_sumz > limit {
//         // max_sum = -1;
//         return max_sum.max(knapsack(0, 0, i + 1, limit, v, w));
//     }
//
//     max_sum.max(knapsack(v_sumz, w_sumz, i + 1, limit, v, w))
// }

// params add selected bool vec, base case and inside it (which is the last point of the recursive
// loop) print the selected, add the selected[i] and recurence with sum + w[i], exculde and
// recurence sum only,

fn all_subsets_string(mut res: &mut String, idx: usize) {
    if idx == 4 {
        println!("{:?}", res);
        return;
    }

    res.replace_range(idx..idx + 1, "0");
    all_subsets_string(&mut res, idx + 1);
    res.replace_range(idx..idx + 1, "1");
    all_subsets_string(&mut res, idx + 1);
    res.replace_range(idx..idx + 1, "2");
    all_subsets_string(&mut res, idx + 1);
    // res.replace_range(idx..idx + 1, " ");
}

fn all_subsets(res: &mut Vec<i32>, idx: usize) {
    if idx == 4 {
        println!("{:?}", res);
        return;
    }

    res[idx] = 0;
    all_subsets(res, idx + 1);
    res[idx] = 1;
    all_subsets(res, idx + 1);
    // res[idx] = -1;
}

fn knapsack_2(
    w: &Vec<i32>,
    v: &Vec<i32>,
    limit: i32,
    i: usize,
    len: usize,
    w_sum: i32,
    selected: &mut Vec<bool>,
) {
    if i >= len {
        if w_sum <= limit {
            println!("{:?}", selected);
        }
        return;
    }

    selected[i] = true;
    knapsack_2(w, v, limit, i + 1, len, w_sum + w[i], selected);

    // Exclude the current item from the knapsack (by not adding its weight to w_sum)
    selected[i] = false;
    knapsack_2(w, v, limit, i + 1, len, w_sum, selected);
}

fn knapsack_69(
    w: &Vec<i32>,
    v: &Vec<i32>,
    i: usize,
    len: usize,
    remaining: i32,
    memory: &mut Vec<Vec<Option<i32>>>,
) -> i32 {
    if i >= len {
        return 0;
    }

    if let Some(ret) = memory[i][remaining as usize] {
        println!("hello");
        return ret;
    }

    println!("memory={:#?}", memory);

    // leave
    let choice1 = knapsack_69(w, v, i + 1, len, remaining, memory);
    let mut choice2 = 0;

    if remaining >= w[i] {
        // pick
        choice2 = v[i] + knapsack_69(w, v, i + 1, len, remaining - w[i], memory);
    }

    memory[i][remaining as usize] = Some(choice1.max(choice2));
    memory[i][remaining as usize].unwrap()
}

fn knapsack(
    w: &Vec<i32>,
    v: &Vec<i32>,
    limit: i32,
    i: usize,
    len: usize,
    sum: i32,
    selected: &mut Vec<bool>,
) {
    if i >= len {
        if sum <= limit {
            // Only print if the total weight is within the limit
            for &is_selected in selected.iter() {
                if is_selected {
                    print!("1 ");
                } else {
                    print!("0 ");
                }
            }
            println!();
        }
        return;
    }

    // Include the current item in the knapsack
    selected[i] = true;
    knapsack(w, v, limit, i + 1, len, sum + w[i], selected);

    // Exclude the current item from the knapsack
    selected[i] = false;
    knapsack(w, v, limit, i + 1, len, sum, selected);
}

fn main() {
    let weights = vec![10, 4, 20, 5, 7];
    let values = vec![10, 15, 3, 1, 4];
    let knapsack_limit = 12;

    let n = weights.len();
    let mut selected = vec![false; n];

    knapsack(&weights, &values, knapsack_limit, 0, n, 0, &mut selected);
    let mut memory: Vec<Vec<Option<i32>>> = vec![vec![None; 20]; 20];
    let x: Option<i32> = Some(5);
    let res_69 = knapsack_69(&weights, &values, 0, n, knapsack_limit, &mut memory);
    println!("{}", res_69);
    // knapsack_2(&weights, &values, knapsack_limit, 0, n, 0, &mut selected);
    // let mut res = vec![-1, -1, -1, -1];
    // all_subsets(&mut res, 0);
    // let mut res = String::from("    ");
    // all_subsets_string(&mut res, 0);
    // 1 0 0 0 0
    // 0 1 0 1 0
    // 0 1 0 0 1
    // 0 1 0 0 0
    // 0 0 0 1 1
    // 0 0 0 1 0
    // 0 0 0 0 1
    // 0 0 0 0 0
}
