use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    // let infix = String::from("1+3*5-8/2");
    // println!("{}", infix_to_postfix(infix));
    // // 135*+82/-
    // let infix = String::from("2+3-((5+2)*3)");
    // println!("{}", infix_to_postfix(infix));
    // // ● Final expression 23+52+3*-
    //
    // let infix = String::from("4+3+2");
    // println!("{}", infix_to_postfix(infix));
    //
    let infix = String::from("4^3^2");
    println!("{}", infix_to_postfix(infix));
    //
    // // ○ a+b*(c^d-e)^(f+G*h)-i ⇒ abcd^e-fGh*+^*+i-
    //
    // let infix = String::from("a+b*(c^d-e)^(f+G*h)-i");
    // println!("{}", infix_to_postfix(infix));

    // let infix = "(1+(4+5+2)-3)+(6+8)".to_string();
    // println!("{}", infix_to_postfix(infix));
    // let infix = "3+2-9".to_string();
    // println!("{}", infix_to_postfix(infix));
}

fn infix_to_postfix(infix: String) -> String {
    let mut infix = infix.chars().collect::<Vec<char>>();
    infix.retain(|f| f != &' ');
    let hm = HashMap::from([('+', 1), ('-', 1), ('*', 2), ('/', 2), ('(', 0), ('^', 3)]);
    let mut operations_stack: Vec<char> = Vec::new();
    let mut rezult = String::with_capacity(infix.len());
    for num in infix {
        if num.is_digit(10) || num.is_alphabetic() {
            rezult.push(num);
        } else if num == '(' {
            operations_stack.push('(');
        } else if num == ')' {
            while let Some(pop) = operations_stack.pop() {
                if pop == '(' {
                    break;
                }
                rezult.push(pop);
            }
        } else {
            while !operations_stack.is_empty() && hm[&num] <= hm[operations_stack.last().unwrap()] {
                if hm[operations_stack.last().unwrap()] == 3 && hm[&num] == 3 {
                    break;
                }
                rezult.push(operations_stack.pop().unwrap());
            }
            operations_stack.push(num);
        }
    }

    // dump the rest
    while let Some(pop) = operations_stack.pop() {
        rezult.push(pop);
    }

    println!("{}", evaluate_postfix(rezult.clone()));

    rezult
}

fn evaluate_postfix(postfix: String) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    for token in postfix.chars() {
        if let Some(num) = token.to_digit(10) {
            stack.push(num as i32);
        } else {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            let result = match token {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                '^' => b.pow(a as u32),
                _ => unreachable!(),
            };
            stack.push(result);
        }
    }

    stack.pop().unwrap()
}
