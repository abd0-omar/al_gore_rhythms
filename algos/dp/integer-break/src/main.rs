fn main() {
    println!("Hello, world!");
    let n = 7;
    println!("{}", integer_break(n));
}

fn integer_break(n: i32) -> i32 {
    _integer_break(n)
}

fn _integer_break(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let mut res = n;
    for i in 1..n {
        res = res.max(i * _integer_break(n - i));
    }

    res
}
