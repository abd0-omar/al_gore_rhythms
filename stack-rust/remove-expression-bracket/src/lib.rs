pub fn remove_brackets(input: String) -> String {
    let mut stack = Vec::new();
    let mut rezult = String::with_capacity(input.len());
    let mut last_sign_nega = false;
    for input_char in input.chars() {
        match input_char {
            '(' => stack.push(0),
            ')' => {
                let _ = stack.pop();
            }
            '+' => {
                if last_sign_nega {
                    if stack.len() % 2 == 1 {
                        rezult.push('-')
                    } else {
                        rezult.push('+')
                    }
                } else {
                    rezult.push('+')
                }
                if stack.len() == 0 {
                    last_sign_nega = false;
                }
            }
            '-' => {
                if last_sign_nega {
                    if stack.len() % 2 == 1 {
                        rezult.push('+')
                    } else {
                        rezult.push('-')
                    }
                } else {
                    rezult.push('-')
                }
                if stack.len() == 0 {
                    last_sign_nega = true;
                }
            }
            _ => rezult.push(input_char),
        }
    }
    rezult
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let result = remove_brackets(String::from("1+2-3-4+5-6-7+8"));
        assert_eq!(result, String::from("1+2-3-4+5-6-7+8"));
    }

    #[test]
    fn it_works2() {
        let result = remove_brackets(String::from("9-(2+3)"));
        assert_eq!(result, String::from("9-2-3"));
    }

    #[test]
    fn it_works3() {
        let result = remove_brackets(String::from("9-(2-3)"));
        assert_eq!(result, String::from("9-2+3"));
    }

    #[test]
    fn it_works4() {
        let result = remove_brackets(String::from("9+(2-3)"));
        assert_eq!(result, String::from("9+2-3"));
    }

    #[test]
    fn it_works5() {
        let result = remove_brackets(String::from("1-(2-3-(4+5))-6-(7-8)"));
        assert_eq!(result, String::from("1-2+3+4+5-6-7+8"));
    }

    #[test]
    fn it_works6() {
        let result = remove_brackets(String::from("1-(2-3-(4+5)+6-7)"));
        assert_eq!(result, String::from("1-2+3+4+5-6+7"));
    }

    #[test]
    fn it_works7() {
        let result = remove_brackets(String::from("1-(2-3-(4+5-(6-7)))"));
        assert_eq!(result, String::from("1-2+3+4+5-6+7"));
    }
}
