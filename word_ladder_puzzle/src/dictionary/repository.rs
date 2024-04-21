use std::{collections::HashMap, fs};

use crate::dictionary::WordDictionary;

pub trait DictionaryRepo {
    fn get_english_dictionary(&self) -> WordDictionary;
}
pub struct JsonDictionaryRepo {
    file_path: String,
}

impl JsonDictionaryRepo {
    pub fn create(file_path: String) -> JsonDictionaryRepo {
        JsonDictionaryRepo { file_path }
    }
}

impl DictionaryRepo for JsonDictionaryRepo {
    fn get_english_dictionary(&self) -> WordDictionary {
        let file_string = fs::read_to_string(&self.file_path).unwrap();

        let map: HashMap<String, serde_json::Value> = serde_json::from_str(&file_string).unwrap();
        WordDictionary {
            words: map.keys().cloned().collect(),
        }
    }
}
