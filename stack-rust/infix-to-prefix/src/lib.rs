use std::collections::HashMap;

pub fn infix_to_prefix(infix: String) -> String {
    let reversed_infix: String = infix.chars().rev().collect();
    let mut reversed_infix = reversed_infix.chars().collect::<Vec<char>>();
    reversed_infix.retain(|&c| c != ' ');

    let reversed_postfix = reversed_infix_to_postfix(&reversed_infix);

    reversed_postfix.chars().rev().collect()
}

fn reversed_infix_to_postfix(reversed_infix: &Vec<char>) -> String {
    let hm = HashMap::from([
        ('+', 1),
        ('-', 1),
        ('*', 2),
        ('/', 2),
        ('(', 0),
        (')', 0),
        ('^', 3),
    ]);
    let mut operations_stack: Vec<char> = Vec::new();
    let mut postfix = String::new();

    for &token in reversed_infix.iter() {
        if token.is_digit(10) || token.is_alphabetic() {
            postfix.push(token);
        } else if token == ')' {
            operations_stack.push(token);
        } else if token == '(' {
            while let Some(op) = operations_stack.pop() {
                if op == ')' {
                    break;
                }
                postfix.push(op);
            }
        } else {
            while !operations_stack.is_empty()
                && (hm[&token] < hm[operations_stack.last().unwrap()] || token == '^')
            {
                if let Some(pop) = operations_stack.pop() {
                    postfix.push(pop);
                }
            }
            operations_stack.push(token);
        }
    }

    while let Some(op) = operations_stack.pop() {
        postfix.push(op);
    }

    postfix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = infix_to_prefix("1+2".to_string());
        assert_eq!(result, String::from("+12"));
    }

    #[test]
    fn it_works2() {
        let result = infix_to_prefix("1+2*3".to_string());
        assert_eq!(result, String::from("+1*23"));
    }

    #[test]
    fn it_works3() {
        let result = infix_to_prefix("9-2+3".to_string());
        assert_eq!(result, String::from("+-923"));
    }

    #[test]
    fn it_works4() {
        let result = infix_to_prefix("4^3^2".to_string());
        assert_eq!(result, String::from("^4^32"));
    }

    #[test]
    fn it_works5() {
        let result = infix_to_prefix("1+2+3".to_string());
        assert_eq!(result, String::from("++123"));
    }

    #[test]
    fn it_works6() {
        let result = infix_to_prefix("1+2*3".to_string());
        assert_eq!(result, String::from("+1*23"));
    }

    #[test]
    fn it_works7() {
        let result = infix_to_prefix("2*3+4".to_string());
        assert_eq!(result, String::from("+*234"));
    }

    #[test]
    fn it_works8() {
        let result = infix_to_prefix("1+3*5-8/2".to_string());
        assert_eq!(result, String::from("-+1*35/82"));
    }
}
