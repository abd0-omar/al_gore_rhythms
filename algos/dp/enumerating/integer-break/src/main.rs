fn main() {
    println!("Hello, world!");
}
pub fn integer_break(n: i32) -> i32 {
    if n == 2 || n == 3 {
        return n - 1;
    }
    let mut memory = [None; 58];
    _integer_break(n, &mut memory)
}

pub fn _integer_break(n: i32, memory: &mut [Option<i32>; 58]) -> i32 {
    if n == 1 {
        return 1;
    }

    if let Some(ret) = memory[n as usize] {
        return ret;
    }

    let mut ret = n;
    for i in 1..n {
        ret = ret.max(i * integer_break(n - i));
    }
    memory[n as usize] = Some(ret);
    ret
}
