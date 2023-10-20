fn main() {
    println!("Hello, world!");
    let dist = vec![3, 5, 7, 4, 5];
    let speed = vec![2, 3, 6, 3, 2];
    // Output: 1
    let output = eliminate_maximum(dist, speed);
    println!("{}", output);
}

pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
    let n = dist.len();
    let mut dist = dist;
    let mut time: Vec<f32> = Vec::with_capacity(n);

    for i in 0..n {
        time.push(dist[i] as f32 / speed[i] as f32);
    }

    time.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut rez = 0;

    for i in 0..n {
        if i as f32 >= time[i] {
            break;
        }
        rez += 1;
    }

    // for i in 0..n {
    //     if time[i] <= 0.0 {
    //         break;
    //     }
    //     rez += 1;
    //     println!("DEBUGPRINT[2]: main.rs:27: time={:#?}", time);
    //     for j in i + 1..n {
    //         dist[j] -= speed[j];
    //         println!("DEBUGPRINT[4]: main.rs:29: dist={:#?}", dist);
    //         time[j] = dist[j] as f32 / speed[j] as f32;
    //         println!("DEBUGPRINT[3]: main.rs:27: time={:#?}", time);
    //     }
    // }
    println!("DEBUGPRINT[1]: main.rs:11: time={:#?}", time);

    rez
}
