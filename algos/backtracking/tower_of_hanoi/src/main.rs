fn main() {
    println!("Hello, world!");
    // put the rezult in the middle
    tower_of_hanoi(3, 'A', 'B', 'C');
    // put the rezult in the end
    tower_of_hanoi(3, 'A', 'C', 'B');
}

fn tower_of_hanoi(n: u32, from: char, to: char, aux: char) {
    if n == 0 {
        return;
    }

    // put n - 1 from A to C
    tower_of_hanoi(n - 1, from, aux, to);
    // put n from A to B
    println!("moved disk {} from {} to {} ", n, from, to);
    // put n - 1 from C to B
    tower_of_hanoi(n - 1, aux, to, from);
    // the power of WHAT not HOW!
}
