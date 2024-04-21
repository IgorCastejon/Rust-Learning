use crate::dictionary::{repository::DictionaryRepo, WordDictionary};

mod tests;

pub struct ValidWordChecker {
    dict: WordDictionary,
}

impl ValidWordChecker {
    pub fn check_valid_word(&self, word: &str) -> bool {
        let is_word_a_dictionary_word: bool = self.dict.has_word(word);

        is_word_a_dictionary_word
    }

    pub fn create(repo: Box<dyn DictionaryRepo>) -> ValidWordChecker {
        ValidWordChecker {
            dict: repo.get_english_dictionary(),
        }
    }
}
