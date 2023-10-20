fn main() {
    println!("Hello, world!");
    // Input example: axz, axa, zzz, abc, abe
    // ● Output: abc, abe, axz, axa, zzz
    let mut input = vec![
        "axz".to_string(),
        "axa".to_string(),
        "zzz".to_string(),
        "abc".to_string(),
        "abe".to_string(),
    ];

    count_sort(&mut input);

    println!("DEBUGPRINT[4]: main.rs:13: input={:#?}", input);

    //coach's implementation is much cleaner consider flattining the array
}

fn count_sort(array: &mut Vec<String>) {
    let mut freq: Vec<Vec<Vec<String>>> = vec![vec![vec![]; 26]; 26];

    for i in 0..array.len() {
        let chars = &mut array[i][0..2].chars();
        let first_letter = chars.next().unwrap() as u8 - b'a';
        // println!(
        //     "DEBUGPRINT[1]: main.rs:11: first_letter={:#?}",
        //     first_letter
        // );

        let second_letter = chars.next().unwrap() as u8 - b'a';
        // println!(
        //     "DEBUGPRINT[2]: main.rs:27: second_letter={:#?}",
        //     second_letter
        // );

        freq[first_letter as usize][second_letter as usize].push(array[i].clone());
        // println!("DEBUGPRINT[3]: main.rs:33: freq={:#?}", freq);
    }

    let mut idx = 0;

    for first_vec in freq {
        for second_vec in first_vec {
            for word in second_vec {
                array[idx] = word.clone();
                idx += 1;
            }
        }
    }
}
