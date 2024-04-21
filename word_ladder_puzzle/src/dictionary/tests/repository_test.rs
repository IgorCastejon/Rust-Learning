#[cfg(test)]
use crate::dictionary::repository::DictionaryRepo;
use crate::dictionary::repository::JsonDictionaryRepo;

#[test]
fn when_get_english_dictionary_then_should_contain_over_370k_words() {
    let dictionary_repo: Box<dyn DictionaryRepo> = Box::new(JsonDictionaryRepo::create(
        String::from("./data/words_dictionary.json"),
    ));
    let dictionary = dictionary_repo.get_english_dictionary();
    assert!(dictionary.number_of_words().ge(&370000_u64));
}
