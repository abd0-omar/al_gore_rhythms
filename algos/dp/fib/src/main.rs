fn main() {
    println!("Hello, world!");
    let mut memory: [Option<i64>; 81] = [None; 81];
    println!("{}", fib(80, &mut memory));
}

fn fib(n: i64, memory: &mut [Option<i64>]) -> i64 {
    if n <= 1 {
        // I should return n to return 0
        return 1;
    }
    // let memory[n as usize]
    //     .unwrap_or_else(|| memory[n as usize] = Some(fib(n - 1, memory) + fib(n - 2, memory)))
    if let Some(num) = memory[n as usize] {
        return num;
    }

    // memory[n as usize] = if let Some(var_name) = Some(fib(n - 1, memory) + fib(n - 2, memory)) {
    //     return var_name.unwrap();
    // }
    // match memory[n as usize] {
    //     Some(num) => return num,
    //     None => memory[n as usize] = Some(fib(n - 1, memory) + fib(n - 2, memory)),
    // };
    let result = fib(n - 1, memory) + fib(n - 2, memory);
    memory[n as usize] = Some(result);
    result
}
