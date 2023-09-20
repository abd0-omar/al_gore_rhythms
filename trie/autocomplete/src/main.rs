use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    child: [Option<Box<Trie>>; 26],
    is_leaf: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            child: Default::default(),
            is_leaf: false,
        }
    }

    fn insert(&mut self, str: &str) {
        let mut trie = self;

        for c in str.as_bytes() {
            let index: usize = (c - b'a') as usize;
            println!("DEBUGPRINT[1]: main.rs:21: index={:#?}", index);
            if trie.child[index].is_none() {
                trie.child[index] = Some(Box::new(Trie::new()));
            }
            trie = trie.child[index].as_mut().unwrap();
        }

        trie.is_leaf = true;
    }

    fn auto_complete(&self, str: &str, res: &mut Vec<String>) {
        let mut trie = self;

        for c in str.as_bytes() {
            let i = (c - b'a') as usize;
            match trie.child[i].is_some() {
                true => {
                    trie = trie.child[i].as_ref().unwrap();
                }
                false => return,
            }
        }
        trie.list_tree(str, res);
    }

    fn list_tree(&self, cur_str: &str, res: &mut Vec<String>) {
        if self.is_leaf {
            res.push(cur_str.to_string());
        }

        for i in 0..26 {
            if self.child[i].is_some() {
                let ch: char = (i as u8 + b'a') as char;
                let pushed = format!("{}{}", cur_str, ch);
                println!("DEBUGPRINT[2]: main.rs:42: pushed={:#?}", pushed);
                self.child[i].as_ref().unwrap().list_tree(&pushed, res);
            }
        }
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("abcd");
    trie.insert("ab");
    trie.insert("abx");
    trie.insert("abyz");
    trie.insert("xyz");
    trie.insert("a");
    trie.insert("bcd");

    let mut v: Vec<String> = vec![];

    trie.auto_complete("ab", &mut v);

    for word in v.iter() {
        println!("{}", word);
    }
}
