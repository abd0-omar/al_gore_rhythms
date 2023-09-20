use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    // child: HashMap<char, Box<Trie>>, //doesn't need to be a pointer cuz rust handles memory
    //through ownership
    child: HashMap<char, Trie>,
    is_leaf: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            child: HashMap::new(),
            is_leaf: false,
        }
    }

    fn insert(&mut self, str: &str) {
        let mut trie = self;

        for c in str.chars() {
            // trie = trie.child.entry(c).or_insert(Box::new(Trie::new()));
            trie = trie.child.entry(c).or_insert(Trie::new());
            println!("{:?}", trie);
        }
        trie.is_leaf = true;
    }

    fn word_exist(&self, str: &str) -> bool {
        let mut trie = self;

        for c in str.chars() {
            match trie.child.get(&c) {
                Some(child_trie) => trie = child_trie,
                None => return false,
            }
        }
        trie.is_leaf
    }
}

fn main() {
    let mut trie = Trie::new();

    trie.insert("abdo");
    trie.insert("abde");

    println!("Does 'abdo' exist in the trie? {}", trie.word_exist("abdo"));
    println!("Does 'abde' exist in the trie? {}", trie.word_exist("abde"));
    println!("Does 'abc' exist in the trie? {}", trie.word_exist("abc"));
}
