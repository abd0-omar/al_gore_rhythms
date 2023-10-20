fn main() {
    println!("Hello, world!");
    let mut v = vec![2, 4, 6, 8, 5, 9, 3];
    // let mut v = vec![4, 2];

    for i in 1..v.len() {
        let cpy = v[i];
        let mut j = i;

        while j > 0 && cpy < v[j - 1] {
            v[j] = v[j - 1];
            j -= 1;
        }

        v[j] = cpy;
    }

    let mut v = vec![2, 4, 6, 8, 5, 9, 3];

    for left in 0..v.len() {
        let mut smallest = left;
        for right in (left + 1)..v.len() {
            if v[right] < v[smallest] {
                smallest = right;
            }
        }
        v.swap(left, smallest);
    }

    println!("{:?}", v);
}
