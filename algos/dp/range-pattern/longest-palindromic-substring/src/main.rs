fn main() {
    println!("Hello, world!");
    let s = "babad".to_string();
    println!("{}", longest_palindrome(s));
}

pub fn longest_palindrome(s: String) -> String {
    let mut memory = [[None; 1000]; 1000];
    let mut max = 0;
    let mut max_string = String::new();
    for i in 0..s.len() {
        for j in i + 1..=s.len() {
            if _longest_palindrome(s.as_bytes(), i, j, &mut memory) {
                let n = j - i;
                if n > max {
                    max = n;
                    max_string = s[i..j].to_string();
                }
            }
        }
    }
    max_string
}

pub fn _longest_palindrome(
    s: &[u8],
    st: usize,
    end: usize,
    memory: &mut [[Option<bool>; 1000]; 1000],
) -> bool {
    if st >= end {
        return true;
    }

    if s[st] != s[end - 1] {
        return false;
    }

    if let Some(ret) = memory[st][end] {
        return ret;
    }

    let result = _longest_palindrome(s, st + 1, end - 1, memory);
    memory[st][end] = Some(result);
    memory[st][end].unwrap()
}
