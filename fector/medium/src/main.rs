fn main() {
    let mut v: Vec<i32> = vec![0, 1, 2, 3, 4];
    right_rotate(&mut v);
    println!("{:?}", v);
    left_rotate(&mut v);
    println!("{:?}", v);
    left_rotate(&mut v);
    println!("{:?}", v);
}

fn right_rotate(v: &mut Vec<i32>) {
    let temp = v[v.len() - 1];
    let n = v.len();
    for i in (1..n).rev() {
        v[i] = v[i - 1];
    }
    v[0] = temp;
}

fn left_rotate(v: &mut Vec<i32>) {
    let temp = v[0];

    let n = v.len();

    for i in 0..n - 1 {
        v[i] = v[i + 1];
    }

    v[n - 1] = temp;
}
