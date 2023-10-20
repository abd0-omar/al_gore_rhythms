fn main() {
    let dist = vec![3, 2, 4];
    let speed = vec![5, 3, 2];
    // Output: 1
    println!("{}", eliminate_maximum(dist, speed));
}

pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
    let n = dist.len();
    let mut pos = (0..n).collect::<Vec<usize>>();
    println!("DEBUGPRINT[1]: main.rs:8: pos={:#?}", pos);

    pos.sort_unstable_by(|&a, &b| dist[a].cmp(&dist[b]));

    println!("DEBUGPRINT[2]: main.rs:11: pos={:#?}", pos);

    let mut i = 0;
    let mut sum = 0;

    while i < n {
        println!("DEBUGPRINT[4]: main.rs:20: i={:#?}", i);
        println!(
            "DEBUGPRINT[4]: main.rs:20: dist - speed={:#?}",
            dist[pos[i]] - speed[pos[i]]
        );

        if i as i32 >= (dist[pos[i]] - speed[pos[i]]) {
            println!("DEBUGPRINT[3]: main.rs:21: i={:#?}", i);
            break;
        }
        sum += 1;
        i += 1;
    }

    sum
}
