use std::{
    char,
    collections::{HashMap, HashSet, VecDeque},
    fs::copy,
};

use crate::{
    dictionary::repository::{DictionaryRepo, JsonDictionaryRepo},
    valid_word_checker::{self, ValidWordChecker},
};

mod tests;

#[derive(Debug, PartialEq)]
pub enum WordChainsError {
    InvalidStartWord,
    InvalidEndWord,
    InvalidStartAndEndWords,
    WordsShouldHaveSameSize,
    NoValidChain,
}

pub struct WordChains {
    valid_word_checker: ValidWordChecker,
}

impl WordChains {
    pub fn create() -> WordChains {
        let english_dict_repo: Box<dyn DictionaryRepo> = Box::new(JsonDictionaryRepo::create(
            String::from("./data/words_dictionary.json"),
        ));
        let valid_word_checker = valid_word_checker::ValidWordChecker::create(english_dict_repo);
        WordChains { valid_word_checker }
    }

    fn check_if_words_are_valid(&self, from: &String, to: &String) -> Result<(), WordChainsError> {
        let is_start_word_valid: bool = self.valid_word_checker.check_valid_word(from);
        let is_end_word_valid: bool = self.valid_word_checker.check_valid_word(to);

        match (is_start_word_valid, is_end_word_valid) {
            (false, false) => Err(WordChainsError::InvalidStartAndEndWords),
            (false, true) => Err(WordChainsError::InvalidStartWord),
            (true, false) => Err(WordChainsError::InvalidEndWord),
            (true, true) => {
                if from.len() != to.len() {
                    return Err(WordChainsError::WordsShouldHaveSameSize);
                }
                Ok(())
            }
        }
    }

    pub fn chain(&self, from: String, to: String) -> Result<Vec<String>, WordChainsError> {
        self.check_if_words_are_valid(&from, &to)?;

        let lowercase_start_word = from.to_ascii_lowercase();
        let lowercase_end_word = to.to_ascii_lowercase();

        if lowercase_start_word == lowercase_end_word {
            return Ok(vec![]);
        }

        self.get_chain_path(&lowercase_start_word, &lowercase_end_word)
    }

    fn get_chain_path(
        &self,
        start_word: &String,
        end_word: &String,
    ) -> Result<Vec<String>, WordChainsError> {
        let valid_one_letter_change_words_for_start_word: Vec<String> =
            self.get_all_valid_one_letter_change_words_from_word(start_word);

        let mut frontier: VecDeque<String> =
            VecDeque::from(valid_one_letter_change_words_for_start_word);

        let mut seen_words = HashSet::from([start_word.clone()]);

        let mut predecessor_words: HashMap<String, String> = HashMap::new();
        frontier.iter().for_each(|x| {
            predecessor_words.insert(x.clone(), start_word.clone());
        });

        let mut current_word = start_word.clone();

        while !frontier.is_empty() {
            let possible_one_letter_change = frontier.pop_front().unwrap();
            if seen_words.contains(&possible_one_letter_change) {
                continue;
            }
            seen_words.insert(possible_one_letter_change.clone());

            current_word = possible_one_letter_change.clone();
            if possible_one_letter_change == end_word.clone() {
                break;
            } else {
                let valid_one_letter_change_words_for_possible_one_letter_change = self
                    .get_all_valid_one_letter_change_words_from_word(&possible_one_letter_change);

                let filtered_valid_one_letter_change_words_for_possible_one_letter_change: Vec<
                    String,
                > = valid_one_letter_change_words_for_possible_one_letter_change
                    .iter()
                    .filter(|x| !seen_words.contains(x.clone()))
                    .cloned()
                    .collect();

                filtered_valid_one_letter_change_words_for_possible_one_letter_change
                    .iter()
                    .for_each(|x| {
                        if !predecessor_words.contains_key(x) {
                            predecessor_words.insert(x.clone(), current_word.clone());
                        }
                    });
                frontier
                    .extend(filtered_valid_one_letter_change_words_for_possible_one_letter_change);
            }
        }

        if frontier.is_empty() {
            return Err(WordChainsError::NoValidChain);
        }

        let mut backtrack_chain: Vec<String> = Vec::new();
        loop {
            let backtrack_word = predecessor_words.get(&current_word).unwrap();
            if backtrack_word == start_word {
                break;
            }
            current_word = backtrack_word.clone();

            backtrack_chain.push(backtrack_word.clone());
        }

        let mut backtrack_chain_copy = backtrack_chain.clone();
        backtrack_chain_copy.reverse();
        Ok(backtrack_chain_copy)
    }

    fn get_all_valid_one_letter_change_words_from_word(&self, word: &String) -> Vec<String> {
        let mut all_valid_one_letter_change_words: Vec<String> = Vec::new();
        for (pos, original_char) in word.chars().enumerate() {
            let all_valid_one_letter_change_words_for_this_position =
                self.get_all_valid_one_letter_change_words_for_position(original_char, word, pos);
            all_valid_one_letter_change_words
                .extend(all_valid_one_letter_change_words_for_this_position)
        }
        all_valid_one_letter_change_words
    }

    fn get_all_valid_one_letter_change_words_for_position(
        &self,
        original_char: char,
        word: &String,
        pos: usize,
    ) -> Vec<String> {
        let mut all_valid_one_letter_change_words_for_this_position: Vec<String> = Vec::new();

        for letter in 'a'..='z' {
            if letter == original_char {
                continue;
            }

            let modified_one_letter = Self::replace_char(word, pos, letter);

            if self.valid_word_checker.check_valid_word(word) {
                all_valid_one_letter_change_words_for_this_position.push(modified_one_letter);
            }
        }
        all_valid_one_letter_change_words_for_this_position
    }

    fn replace_char(original: &String, pos: usize, new_char: char) -> String {
        let mut copy_original = original.clone();
        copy_original.replace_range(pos..(pos + 1), new_char.to_string().as_str());
        copy_original
    }
}
