use std::collections::HashMap;

#[derive(Debug)]
pub struct Trie {
    pub head: Node,
    pub encodings: HashMap<char, char>,
}

#[derive(Default, Debug)]
pub struct Node {
    pub is_terminal: bool,
    // default false
    pub strings: Vec<String>,
    // since 2 string can be equivalent ("a" == -a) for trie
    pub children: HashMap<char, Node>,
}

impl Trie {
    pub fn new(encodings: HashMap<char, char>) -> Self {
        let head = Node::default();
        Self { head, encodings }
    }

    pub fn insert(&mut self, value: String) {
        let mut node = &mut self.head as *mut Node;
        for mut next_char in value.chars() {
            if next_char == '-' || next_char == '"' {
                continue;
            }
            next_char = next_char.to_ascii_lowercase();
            next_char = *self.encodings.get(&next_char).unwrap();
            unsafe {
                match (*node).children.get_mut(&next_char) {
                    None => {
                        let new_node = Node::default();
                        (*node).children.insert(next_char, new_node);
                        node = (*node).children.get_mut(&next_char).unwrap() as *mut Node;
                    }
                    Some(n) => {
                        node = n as *mut Node;
                    }
                }
            }
        }
        unsafe {
            (*node).is_terminal = true;
            (*node).strings.push(value);
        }
    }

    pub unsafe fn find(&mut self, key: String) -> Option<Vec<String>> {
        let mut node = &mut self.head as *mut Node;
        for mut next_char in key.chars() {
            next_char = next_char.to_ascii_lowercase();

            if next_char == '-' || next_char == '/' {
                continue;
            }
            match (*node).children.get_mut(&next_char) {
                None => {
                    return None;
                }
                Some(link) => {
                    node = link as *mut Node;
                }
            }
        }
        Some((*node).strings.clone())
    }
}