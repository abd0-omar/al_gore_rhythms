fn main() {
    println!("Hello, world!");
    let str1 = String::from("abazdc");
    let str2 = String::from("bacbadz");
    println!("{}", lcs(str1, str2));
}

fn lcs(str1: String, str2: String) -> i32 {
    _lcs(str1.as_bytes(), str2.as_bytes(), 0, 0)
}

fn _lcs(str1: &[u8], str2: &[u8], idx1: usize, idx2: usize) -> i32 {
    if idx1 == str1.len() || idx2 == str2.len() {
        return 0;
    }

    // let mut choice1 = 0;
    if str1[idx1] == str2[idx2] {
        return 1 + _lcs(str1, str2, idx1 + 1, idx2 + 1);
    }

    let choice2 = _lcs(str1, str2, idx1 + 1, idx2);
    let choice3 = _lcs(str1, str2, idx1, idx2 + 1);

    // choice1.max(choice2.max(choice3))
    choice2.max(choice3)
}
