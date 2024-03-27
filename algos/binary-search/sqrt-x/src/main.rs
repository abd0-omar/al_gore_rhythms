fn main() {
    println!("Hello, world!");
    let x = 8;
    println!("{}", my_sqrt(x));
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

// not using eps
pub fn my_sqrt_with_loop(x: i32) -> i32 {
    let x = x as f64;
    let mut start = 0.0;
    let mut end = x as f64;
    // let eps = 1e-9;
    let mut ans = -1f64;

    let mut i = 0;
    while start <= end && i < 1000 {
        let mid = start + (end - start) / 2.0;
        let mult = mid * mid;
        println!("mult={:?}", mult);
        let mult_floor = mult.floor();
        println!("mult_floor={:?}", mult_floor);
        if mult < x {
            println!("mult={:?}", mult);
            ans = mid;
            start = mid + 1.0;
        } else if mult > x {
            end = mid - 1.0;
        }
        if mult_floor == x {
            ans = mid;
        }
        i += 1;
    }

    ans as _
}
