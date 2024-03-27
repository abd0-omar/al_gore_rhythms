fn main() {
    println!("Hello, world!");
}

pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for &asteroid in asteroids.iter() {
        let mut lost = false;

        while !stack.is_empty() && asteroid < 0 && *stack.last().unwrap() > 0 {
            if stack.last().unwrap().abs() == asteroid.abs() {
                stack.pop();
                lost = true;
                break;
            }
            if stack.last().unwrap().abs() < asteroid.abs() {
                stack.pop();
            } else {
                break;
            }
        }

        if !lost && (stack.is_empty() || stack.last().unwrap() < &0 || asteroid > 0) {
            stack.push(asteroid);
        }
    }

    stack
}
