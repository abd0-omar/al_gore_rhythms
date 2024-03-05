fn main() {
    println!("Hello, world!");
    let text1 = "abcde".to_string();
    let text2 = "ace".to_string();
    // Output: 3
    // Explanation: The longest common subsequence is "ace" and its length is 3.
    println!("{}", longest_common_subsequence(text1, text2));
}

pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let mut memory = [[None; 2500]; 2500];
    _longest_common_subsequence(&text1, &text2, 0, 0, &mut memory)
}

pub fn _longest_common_subsequence(
    text1: &String,
    text2: &String,
    idx1: usize,
    idx2: usize,
    memory: &mut [[Option<i32>; 2500]; 2500],
) -> i32 {
    if idx1 == text1.len() || idx2 == text2.len() {
        return 0;
    }

    let mut choice1 = 0;
    let mut choice2 = 0;
    let mut choice3 = 0;

    if let Some(ret) = memory[idx1][idx2] {
        return ret;
    }

    if &text1[idx1..idx1 + 1] == &text2[idx2..idx2 + 1] {
        choice1 = 1 + _longest_common_subsequence(text1, text2, idx1 + 1, idx2 + 1, memory);
    } else {
        choice2 = _longest_common_subsequence(text1, text2, idx1 + 1, idx2, memory);
        choice3 = _longest_common_subsequence(text1, text2, idx1, idx2 + 1, memory);
    }

    memory[idx1][idx2] = Some(choice1.max(choice2.max(choice3)));
    memory[idx1][idx2].unwrap()
}
