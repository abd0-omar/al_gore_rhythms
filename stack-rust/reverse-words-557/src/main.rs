fn main() {
    let s = "Let's take LeetCode contest".to_owned();
    let s = reverse_words(s);
    println!("{s}");
}

pub fn reverse_words(s: String) -> String {
    // s = "Let's take LeetCode contest"
    //"s'teL ekat edoCteeL tsetnoc"
    let mut st = Vec::new();
    let mut str = String::new();
    for c in s.split_whitespace() {
        for i in c.chars() {
            st.push(i);
        }
        for _ in c.chars() {
            let pop = st.last().unwrap().clone();
            str.push(pop);
            st.pop();
        }
        str.push(' ');
    }
    str.trim_end().to_owned()
}
