fn main() {
    println!("Hello, world!");
}

pub fn min_distance(word1: String, word2: String) -> i32 {
    let mut memory = [[None; 2500]; 2500];
    _min_distance(&word1, &word2, 0, 0, &mut memory)
}

fn _min_distance(
    word1: &String,
    word2: &String,
    i: usize,
    j: usize,
    memory: &mut [[Option<i32>; 2500]; 2500],
) -> i32 {
    if i == word1.len() {
        return (word2.len() - j) as i32;
    }

    if j == word2.len() {
        return (word1.len() - i) as i32;
    }

    if let Some(ret) = memory[i][j] {
        return ret;
    }

    if &word1[i..i + 1] == &word2[j..j + 1] {
        memory[i][j] = Some(_min_distance(word1, word2, i + 1, j + 1, memory));
        return memory[i][j].unwrap();
    }

    // change
    let choice2 = 1 + _min_distance(word1, word2, i + 1, j + 1, memory);
    // delete
    let choice3 = 1 + _min_distance(word1, word2, i + 1, j, memory);
    // insert
    let choice4 = 1 + _min_distance(word1, word2, i, j + 1, memory);
    memory[i][j] = Some(choice2.min(choice3.min(choice4)));
    memory[i][j].unwrap()
}
