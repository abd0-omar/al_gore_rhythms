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

        for &c in str.as_bytes() {
            let index = (c - b'a') as usize;

            if trie.child[index].is_none() {
                trie.child[index] = Some(Box::new(Trie::new()));
            }
            trie = trie.child[index].as_mut().unwrap();
        }
        trie.is_leaf = true;
    }

    fn get_all_strings(&self, res: &mut Vec<String>, cur_str: &str) {
        // let mut trie = self;

        if self.is_leaf {
            res.push(cur_str.to_string())
        }

        for i in 0..26 as usize {
            if self.child[i].is_some() {
                let ch: char = (i as u8 + b'a') as char;
                let pushed = format!("{}{}", cur_str, ch);
                self.child[i]
                    .as_ref()
                    .unwrap()
                    .get_all_strings(res, &pushed);
            }
        }

        /*
        for (i, child) in self.child.iter().enumerate() {
            if let Some(c) = child {
                let next_char = (i as u8 + b'a') as char;
                let next_str = format!("{}{}", cur_str, next_char);
                c.get_all_strings(&next_str, res);
            }
        }
        */
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("a");
    trie.insert("ab");
    trie.insert("xyz");
    trie.insert("abcd");
    trie.insert("bcd");

    let mut v: Vec<String> = vec![];

    trie.get_all_strings(&mut v, "");

    for word in v.iter() {
        println!("{}", word);
    }
}
