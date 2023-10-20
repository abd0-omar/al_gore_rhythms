fn main() {
    let n = 5;
    // Output: 2
    // Explanation: Because the 3rd row is incomplete, we return 2.

    println!("{}", arrange_coins1(n));

    let n = 8;
    println!("{}", arrange_coins1(n));
    // Output: 3

    let n = 6;
    println!("{}", arrange_coins1(n));

    let n = 1804289383;
    println!("{}", arrange_coins1(n));
    //60070
}

pub fn possible(mid: i64, n: i64) -> bool {
    let sum = (mid * (mid + 1)) / 2;
    if sum <= n {
        true
    } else {
        false
    }
}

pub fn arrange_coins3(n: i32) -> i32 {
    let mut l = 0;
    let mut r = n;
    let mut ans = -1;
    unimplemented!()
}

pub fn arrange_coins2(n: i32) -> i32 {
    let mut l = 0;
    let mut r = n;
    let mut ans = -1;

    while l <= r {
        let mid = l + (r - l) / 2;

        match possible(mid as i64, n as i64) {
            true => {
                l = mid + 1;
                ans = mid;
            }
            false => r = mid - 1,
        }
    }

    ans
}

pub fn arrange_coins1(n: i32) -> i32 {
    // 1, 2, 3, 4, 5, 6, 7, 8
    // mid(mid-1)/ 2 -> mid must be less than or equal
    // last true

    if n == 1 {
        return 1;
    }

    let mut l = 1;
    let mut r = n;
    let mut ans = -1;

    while l <= r {
        let mid = l + (r - l) / 2;
        if possible(mid as i64, n as i64) {
            l = mid + 1;
            ans = mid;
        } else {
            r = mid - 1;
        }
    }

    ans
}
