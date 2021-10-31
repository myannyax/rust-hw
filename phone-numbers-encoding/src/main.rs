use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::solution::encode_number;
use crate::trie::Trie;

mod solution;
mod trie;

fn reverse_map(digits_coding: &HashMap<char, Vec<char>>) -> HashMap<char, char> {
    let mut chars_coding = HashMap::new();
    for (k, v) in digits_coding {
        for symbol in v {
            chars_coding.insert(*symbol, *k);
        }
    }
    chars_coding
}

fn read_dict(chars_encoding: HashMap<char, char>) -> Trie {
    let file = File::open("dictionary.txt").expect("Fail to open dictionary");
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut trie = Trie::new(chars_encoding);
    while reader.read_line(&mut buf).is_ok() {
        let tmp_string = buf.trim().to_string();
        if tmp_string.is_empty() {
            break;
        }
        trie.insert(tmp_string);
        buf.clear();
    }
    trie
}

fn main() {
    let mut reversed_encoding: HashMap<char, Vec<char>> = HashMap::new();
    reversed_encoding.insert('0', vec!['e']);
    reversed_encoding.insert('1', vec!['j', 'n', 'q']);
    reversed_encoding.insert('2', vec!['r', 'w', 'x']);
    reversed_encoding.insert('3', vec!['d', 's', 'y']);
    reversed_encoding.insert('4', vec!['f', 't']);
    reversed_encoding.insert('5', vec!['a', 'm']);
    reversed_encoding.insert('6', vec!['c', 'i', 'v']);
    reversed_encoding.insert('7', vec!['b', 'k', 'u']);
    reversed_encoding.insert('8', vec!['l', 'o', 'p']);
    reversed_encoding.insert('9', vec!['g', 'h', 'z']);
    let encoding = reverse_map(&reversed_encoding);
    let mut trie = read_dict(encoding);
    let input_file = File::open("input.txt").expect("Can't open input");
    let mut reader = BufReader::new(input_file);
    let mut buf = String::new();
    while reader.read_line(&mut buf).is_ok() {
        if buf.is_empty() {
            break;
        }
        let number = buf.trim().to_string();
        let results = encode_number(&number, &mut trie);
        for i in results {
            if i.trim().is_empty() {
                continue;
            }
            println!("{}: {}", number, i.trim());
        }
        buf.clear();
    }
}