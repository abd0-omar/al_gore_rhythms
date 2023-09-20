fn main() {
    println!("Hello, world!");
    let v = vec![2, 4, 6, 8, 5];

    for i in (0..v.len() - 1).rev() {
        // println!("{}", i);
        let cpy = v[i + 1];
        for j in (1..=i + 1).rev() {
            println!("j: {}, i: {}", j, i);
            // if v[j] <= v[i] {
            //
            // }
        }
    }
}
