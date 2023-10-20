fn main() {
    println!("Hello, world!");
}

pub fn my_sqrt_d(value: f64, eps: f64) -> f64 {
    let mut start = 0.0;
    let mut end = value;

    while end - start > eps {
        let mid = start + (end - start) / 2.0;
        if mid * mid - value < 0.0 {
            start = mid;
        } else {
            end = mid;
        }
    }

    start + 1e-9
}

pub fn my_sqrt(x: i32) -> i32 {
    my_sqrt_d(x as f64, 1e-9) as i32
}
