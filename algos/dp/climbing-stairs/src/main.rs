fn main() {
    println!("Hello, world!");
}

pub fn climb_stairs(n: i32) -> i32 {
    let mut memory = [None; 2500];
    _climb_stairs(n, &mut memory)
}

// fibonacci
pub fn _climb_stairs(n: i32, memory: &mut [Option<i32>; 2500]) -> i32 {
    if n <= 1 {
        return 1;
    }

    if let Some(ret) = memory[n as usize] {
        return ret;
    }

    let mut ret = Box::new(memory[n as usize].unwrap());

    memory[n as usize] = Some(_climb_stairs(n - 1, memory) + _climb_stairs(n - 2, memory));
    memory[n as usize].unwrap()
}
