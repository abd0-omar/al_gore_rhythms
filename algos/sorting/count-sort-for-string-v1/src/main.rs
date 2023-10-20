fn main() {
    //Input example: ziad, belal, adam, baheir, ali
    // ● Output: adam, ali, belal, baheir, ziad
    let mut input = vec![
        "ziad".to_string(),
        "belal".to_string(),
        "adam".to_string(),
        "baheir".to_string(),
        "ali".to_string(),
    ];
    let output = count_sort(&mut input);
    println!("{:?}", input);
}

fn count_sort(array: &mut Vec<String>) {
    let mut freq: Vec<Vec<String>> = vec![vec![]; 26];
    for i in 0..array.len() {
        let first_letter = &array[i][0..1];
        let first_letter = first_letter.chars().next().unwrap();
        let first_letter = first_letter as u8 - b'a';
        println!(
            "DEBUGPRINT[2]: main.rs:13: first_letter={:#?}",
            first_letter
        );

        freq[first_letter as usize].push(array[i].to_string());
        println!("DEBUGPRINT[4]: main.rs:19: freq={:#?}", freq);
    }

    // array.clear();

    let mut idx = 0;
    for vec in freq {
        for word in vec.iter() {
            // array.push(word.clone());
            array[idx] = word.clone();
            idx += 1;
        }
    }
}
