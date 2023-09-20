use std::collections::HashMap;

// struct Trie<'a> {
//     child: HashMap<&'a str, Trie<'a>>,
//     is_leaf: bool,
// }

#[derive(Debug)]
struct Trie {
    child: HashMap<String, Trie>,
    is_leaf: bool,
}
impl Trie {
    fn new() -> Self {
        Trie {
            child: HashMap::new(),
            is_leaf: false,
        }
    }
    //
    // add path
    // one with home/abdo
    // and one with home, abdo
    fn insert_vec(&mut self, path: &Vec<String>) {
        let mut trie = self;

        for word in path.iter() {
            trie = trie.child.entry(word.clone()).or_insert(Trie::new());
        }

        trie.is_leaf = true;
    }

    fn subpath_exist(&self, path: &Vec<String>) -> bool {
        let mut trie = self;

        for word in path.iter() {
            match trie.child.get::<String>(&word) {
                Some(found) => trie = found,
                None => return false,
            }
        }

        return true;
    }

    fn insert_vec_unsplitted(&mut self, path: &String) {
        let mut trie = self;

        for word in path.split('/') {
            // to handle if the first splitted character is `/`
            if word == "" {
                continue;
            }
            println!("{word}");
            trie = trie.child.entry(word.to_string()).or_insert(Trie::new());
        }

        trie.is_leaf = true;
    }

    fn subpath_exist_unsplitted(&self, path: &String) -> bool {
        let mut trie = self;

        for word in path.split('/') {
            // to handle if the first splitted character is `/`
            if word == "" {
                continue;
            }
            match trie.child.get(&word.to_string()) {
                Some(found) => trie = found,
                None => return false,
            }
        }

        return true;
    }

    //ofc there is a way to do it in functional programming style or more idiomatic
    //but I don't know   yet
}

fn main() {
    let mut trie = Trie::new();

    let path1 = vec![
        "home".to_string(),
        "abdo".to_string(),
        "duck".to_string(),
        "al_gore_rhythms".to_string(),
    ];

    trie.insert_vec(&path1);

    let path2 = vec!["home".to_string(), "abdo".to_string(), "luck".to_string()];

    let path3 = vec!["home".to_string(), "abdo".to_string()];

    println!(
        "Does {:?} exist in the trie? {}",
        path2,
        trie.subpath_exist(&path2)
    );

    println!(
        "Does {:?} exist in the trie? {}",
        path3,
        trie.subpath_exist(&path3)
    );

    println!();

    // split with `/`
    let path5 = "/home/abdo/duck/al_gore_rhythms".to_string();

    trie.insert_vec_unsplitted(&path5);

    let path6 = "/home/abdo/luck".to_string();

    let path7 = "/home/abdo".to_string();

    println!(
        "Does {:?} exist in the trie? {}",
        path6,
        trie.subpath_exist_unsplitted(&path6)
    );

    println!(
        "Does {:?} exist in the trie? {}",
        path7,
        trie.subpath_exist_unsplitted(&path7)
    );
}
