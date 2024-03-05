fn main() {
    println!("Hello, world!");
    // let s = "226".to_string();
    // println!("{}", num_decodings(s));
    // let s = "06".to_string();
    // println!("{}", num_decodings(s));
    let s = "27".to_string();
    println!("{}", num_decodings(s));
}

pub fn num_decodings(s: String) -> i32 {
    let mut sum = 0;
    let mut valid = false;
    let mut memory = vec![None; s.len()];
    _num_decodings(s.as_bytes(), 0, &mut valid, &mut sum, &mut memory);
    sum
}

pub fn _num_decodings(
    s: &[u8],
    idx: usize,
    valid: &mut bool,
    sum: &mut i32,
    memory: &mut Vec<Option<i32>>,
) -> () {
    if idx >= s.len() {
        if *valid {
            *valid = false;
            *sum += 1;
        }
        return;
    }

    // Check if the result is already memoized, idk how tbh
    if let Some(result) = memory[idx] {
        *sum += result;
        return;
    }

    // pick one
    if s[idx] != b'0' {
        *valid = true;
        _num_decodings(s, idx + 1, valid, sum, memory);
    } else {
        *valid = false;
    }

    // pick two
    // pick only if valid
    if idx + 1 < s.len() {
        let num = format!("{}{}", s[idx] as char, s[idx + 1] as char);
        if s[idx] != b'0' && num.parse::<u32>().unwrap() <= 26 {
            *valid = true;
            _num_decodings(s, idx + 2, valid, sum, memory);
        }
    } else {
        *valid = false;
    }

    // Memoize the result, still don't know how
    memory[idx] = Some(*sum);
}
