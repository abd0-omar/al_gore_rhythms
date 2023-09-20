fn main() {
    let s = String::from("([)]");

    println!("{}", is_valid(s));
}
pub fn is_valid(s: String) -> bool {
    // Input: s = "()[]{}"
    // Output: true

    let mut st: Vec<char> = Vec::new();
    let mut hs = std::collections::HashMap::new();

    hs.insert(')', '(');
    hs.insert(']', '[');
    hs.insert('}', '{');
    println!("{:?}", hs);

    for c in s.chars() {
        println!("OK");
        println!("{c}");
        if let Some(top) = st.last() {
            if hs.get(&c).is_some() {
                if *top == *hs.get(&c).unwrap() {
                    st.pop();
                    continue;
                }
            }
            // println!("{:?}", top);
            // println!("{:?}", hs.get(&c));
        }
        st.push(c);
    }

    st.is_empty()
}
