use std::collections::HashMap;
#[derive(Debug)]
struct Trie {
    child: HashMap<char, Trie>,
    is_leaf: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            child: HashMap::new(),
            is_leaf: false,
        }
    }

    pub fn insert(&mut self, str: &str) {
        let mut trie = self;
        for char in str.chars() {
            trie = trie.child.entry(char).or_insert(Trie::new());
            // println!("{}", char);
        }
        trie.is_leaf = true;
    }

    pub fn word_exist(&self, str: &str) -> bool {
        let mut trie = self;
        for char in str.chars() {
            if let Some(curr_trie) = trie.child.get(&char) {
                // println!("DEBUGPRINT[2]: main.rs:27: char={:#?}", char);
                trie = curr_trie;
                // println!("DEBUGPRINT[3]: main.rs:30: trie={:#?}", trie);
            } else {
                return false;
            }
        }
        trie.is_leaf
    }

    pub fn list_substrs(&mut self, input: &str, queries: &Vec<&str>) {
        for query in queries {
            let mut trie = Trie::new();

            trie.insert(query);

            for i in 0..input.len() {
                for j in i + 1..=input.len() {
                    let word = &input[i..j];
                    // println!("DEBUGPRINT[1]: main.rs:44: word={:#?}", word);
                    if trie.word_exist(word) {
                        println!("{}", word);
                    }
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let input = "heyabcdtwxyw";

    let queries = vec!["ab", "xy", "t", "yz", "bd"];

    let mut trie = Trie::new();

    trie.list_substrs(input, &queries);
}
