fn main() {
    let s = String::from("abbaca");
    println!("{}", remove_duplicates(s));
}
pub fn remove_duplicates(s: String) -> String {
    //     Input: s = "abbaca"
    // Output: "ca"

    let mut st = Vec::new();

    for c in s.chars() {
        if !st.is_empty() && *st.last().unwrap() == c {
            st.pop();
            continue;
        }
        st.push(c);
    }
    st.iter().collect()
}
