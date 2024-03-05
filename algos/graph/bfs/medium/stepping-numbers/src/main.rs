use std::collections::VecDeque;

fn is_stepping_number(n: i32) -> bool {
    let mut prev_digit = -1;
    let mut num = n;
    while num > 0 {
        let digit = num % 10;
        if prev_digit != -1 && (prev_digit - digit).abs() != 1 {
            return false;
        }
        prev_digit = digit;
        num /= 10;
    }
    true
}

fn stepping_numbers(low: i32, high: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();

    for i in 1..=9 {
        if (low..=high).contains(&i) {
            result.push(i);
        }
        if i > high {
            break;
        }
        queue.push_back(i);
    }

    // while !q.is_empty() {
    while let Some(cur) = queue.pop_front() {
        println!("cur={:?}", cur);
        let last_digit = cur % 10;
        println!("last_digit={:?}", last_digit);

        if last_digit != 9 {
            let next = cur * 10 + (last_digit + 1); // 32 -> 323
                                                    // 1 -> 12
            println!("next={:?}", next);
            if (low..=high).contains(&next) {
                if is_stepping_number(next) {
                    println!("DEBUGPRINT[6]: main.rs:42: next={:#?}", next);
                    result.push(next);
                    queue.push_back(next);
                }
            }
        }

        if last_digit != 0 {
            let next = cur * 10 + (last_digit - 1); // 32 -> 321
                                                    // 1 -> 10
            println!("DEBUGPRINT[7]: main.rs:50: next={:#?}", next);
            if (low..=high).contains(&next) {
                if is_stepping_number(next) {
                    result.push(next);
                    queue.push_back(next);
                }
            }
        }
    }

    result
}

fn main() {
    let low = 0;
    let high = 21;
    let result = stepping_numbers(low, high);
    println!("{:?}", result);
    let low = 10;
    let high = 15;
    let result = stepping_numbers(low, high);
    println!("{:?}", result);
}
