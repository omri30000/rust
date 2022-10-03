use std::{collections::HashMap, collections::HashSet, str::FromStr};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = String::from_str(word).unwrap().to_lowercase();
    let mut result = HashSet::new();

    for possible_anagram in possible_anagrams.iter() {
        if word.is_anagram(&(*possible_anagram).to_lowercase()) {
            result.insert(*possible_anagram);
        }
    }

    result
}

pub trait Anagram {
    fn is_anagram(&self, candidate: &str) -> bool;
}

impl Anagram for String {
    fn is_anagram(&self, candidate: &str) -> bool {
        if *self == *candidate {
            return false;
        }
        let mut original_chars_counter: HashMap<char, usize> = HashMap::new();
        for char in self.chars() {
            *original_chars_counter.entry(char).or_insert(0) += 1;
        }

        let mut candidate_chars_counter: HashMap<char, usize> = HashMap::new();
        for char in candidate.chars() {
            *candidate_chars_counter.entry(char).or_insert(0) += 1;
        }

        original_chars_counter == candidate_chars_counter
    }
}
