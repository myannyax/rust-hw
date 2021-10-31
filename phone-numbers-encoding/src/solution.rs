use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::Add;

use crate::trie::Trie;

fn count_letters(str: &String) -> usize {
    str.chars().filter(|cc| cc.is_alphabetic() || cc.is_alphanumeric()).count()
}

pub fn encode_number(number: &String, dictionary: &mut Trie) -> HashSet<String> {
    let mut my_current_words: HashSet<String> = HashSet::new();
    let mut current_result: HashSet<String> = HashSet::new();
    current_result.insert(String::new());
    let mut prev_is_digit = false;
    let mut should_try_subst_with_digit = false;
    let mut count = 0;
    for char in number.chars() {
        my_current_words.insert(String::new());
        my_current_words = my_current_words.iter().map(|word| word.clone().add(&*char.to_string()))
            .collect();
        if char == '-' || char == '/' {} else {
            count += 1;
            my_current_words = HashSet::from_iter(my_current_words.into_iter().filter(|word|
                unsafe {
                    match dictionary.find((*word).clone()) {
                        // that means word isn't even a prefix of some dictionary strings
                        None => {
                            let mut kek = word.clone();
                            let mut flag_kek = false;
                            kek = kek[0..kek.len() - 1].to_string();
                            while !kek.is_empty() {
                                match dictionary.find((kek).clone()) {
                                    None => {}
                                    Some(x) => {
                                        if !x.is_empty() {
                                            flag_kek = true;
                                            break;
                                        }
                                    }
                                }
                                kek = kek[0..kek.len() - 1].to_string();
                            }
                            if flag_kek == true {
                                should_try_subst_with_digit = true;
                            }
                            false
                        }
                        Some(strings) => {
                            if !strings.is_empty() {
                                let len = count_letters(strings.first().unwrap());
                                let tmp_res: HashSet<String> = strings.iter()
                                    .map(|subst| current_result.iter()
                                        .filter(|str| count_letters(str) + len == count)
                                        .map(move |x| x.clone().add(subst).add(" ")))
                                    .flatten()
                                    .collect();
                                for kek in tmp_res {
                                    current_result.insert(kek.clone());
                                }
                            }
                            true
                        }
                    }
                }));
            if !should_try_subst_with_digit {
                if prev_is_digit {
                    prev_is_digit = false;
                    continue;
                }
                let tmp_res: HashSet<String> = current_result.iter().filter(|str| count_letters(str) + 1 == count)
                    .map(|x| x.clone().add(&*char.to_string()).add(" ")).collect();
                if !tmp_res.is_empty() {
                    prev_is_digit = true;
                }
                for kek in tmp_res {
                    current_result.insert(kek.clone());
                }
            } else {
                prev_is_digit = false;
            }
            should_try_subst_with_digit = false;
        }
    }
    let mut final_result: HashSet<String> = HashSet::new();
    for kek in current_result {
        if count_letters(&kek) == count {
            final_result.insert(kek.clone());
        }
    }
    return final_result;
}