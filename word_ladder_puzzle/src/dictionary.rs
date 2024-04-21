pub mod repository;
mod tests;

use std::collections::HashSet;

pub struct WordDictionary {
    words: HashSet<String>,
}

impl WordDictionary {
    pub fn has_word(&self, word: &str) -> bool {
        let lowercase_word: String = word.to_lowercase();
        self.words.contains(&lowercase_word)
    }
    pub fn number_of_words(&self) -> u64 {
        self.words.len() as u64
    }
}
