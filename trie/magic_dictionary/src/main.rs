struct MagicDictionary {
    child: [Option<Box<MagicDictionary>>; 26],
    is_leaf: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    fn new() -> Self {
        Self {
            child: Default::default(),
            is_leaf: false,
        }
    }

    fn insert(&mut self, dictionary: Vec<String>) {
        for word in dictionary.iter() {
            let mut trie = &mut *self;

            for letter in word.as_bytes() {
                let ch = (letter - b'a') as usize;

                if trie.child[ch].is_none() {
                    trie.child[ch] = Some(Box::new(MagicDictionary::new()));
                }

                trie = trie.child[ch].as_mut().unwrap();
            }

            trie.is_leaf = true;
        }
    }

    fn build_dict(&self, dictionary: Vec<String>) {
        unimplemented!()
    }

    fn search(&self, search_word: String) -> bool {
        unimplemented!()
    }

    fn search_word(&self, search_word: String) -> bool {
        let mut trie = self;

        for c in search_word.as_bytes() {
            let i: usize = (c - b'a') as usize;

            if trie.child[i].is_none() {
                return false;
            }
            trie = trie.child[i].as_ref().unwrap();
        }

        return trie.is_leaf;
    }

    fn word_exist_with_1_change(&self, search_word: String) -> bool {
        let trie = self;
        let mut wordz = search_word.chars().collect::<Vec<_>>();
        let mut flag = false;

        for i in 0..search_word.len() {
            if i == 1 {
                flag = true;
            }
            let cpy = search_word.chars().nth(i).unwrap();
            for alpha in 'a'..='z' {
                if alpha == cpy {
                    continue;
                }
                wordz[i] = alpha;
                // eprintln!("DEBUGPRINT[8]: main.rs:69: wordz={:#?}", wordz);
                let ok = wordz.iter().collect::<String>();
                eprintln!("DEBUGPRINT[10]: main.rs:75: ok={:#?}", ok);
                if trie.search_word(wordz.iter().collect::<String>()) {
                    return true;
                }
            }
            wordz[i] = cpy;
        }
        println!("cool rainbow effect");
        false
    }
}

fn main() {
    let mut trie = MagicDictionary::new();

    let v = vec!["hello".to_string(), "leetcode".to_string()];

    trie.insert(v);

    let one = trie.word_exist_with_1_change("hello".to_string());
    let two = trie.word_exist_with_1_change("hhllo".to_string());
    let three = trie.word_exist_with_1_change("hell".to_string());
    let four = trie.word_exist_with_1_change("leetcoded".to_string());

    eprintln!("DEBUGPRINT[3]: main.rs:61: one={:#?}", one);

    eprintln!("DEBUGPRINT[4]: main.rs:63: two={:#?}", two);

    eprintln!("DEBUGPRINT[5]: main.rs:65: three={:#?}", three);

    eprintln!("DEBUGPRINT[9]: main.rs:90: four={:#?}", four);
    /*

    cout << magicDictionary.search("hello");	 // return False
    cout << magicDictionary.search("hhllo");	 // We can change the second 'h' to 'e' to match "hello" so we return True
    cout << magicDictionary.search("hell");		 // return False
    cout << magicDictionary.search("leetcoded"); // return False
    */
}

/*
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */
