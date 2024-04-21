use crate::dictionary::repository::{DictionaryRepo, JsonDictionaryRepo};
#[cfg(test)]
use crate::valid_word_checker::ValidWordChecker;
use rstest::rstest;

#[test]
fn given_the_word_cat_when_check_that_is_valid_then_returns_true() {
    let word = String::from("cat");

    let english_dict_repo: Box<dyn DictionaryRepo> = get_english_dict_repo();
    let valid_word_checker = ValidWordChecker::create(english_dict_repo);

    assert!(valid_word_checker.check_valid_word(&word));
}

#[test]
fn given_the_word_lead_when_check_that_is_valid_then_returns_true() {
    let word = String::from("lead");

    let english_dict_repo: Box<dyn DictionaryRepo> = get_english_dict_repo();
    let valid_word_checker = ValidWordChecker::create(english_dict_repo);

    assert!(valid_word_checker.check_valid_word(&word));
}

#[rstest]
#[case(String::from("Cat"))]
#[case(String::from("cAt"))]
#[case(String::from("caT"))]
#[case(String::from("CAT"))]
fn given_the_word_cat_with_different_uppercase_and_lowercase_when_check_that_is_valid_then_returns_true(
    #[case] input: String,
) {
    let english_dict_repo: Box<dyn DictionaryRepo> = get_english_dict_repo();
    let valid_word_checker = ValidWordChecker::create(english_dict_repo);

    assert!(valid_word_checker.check_valid_word(&input));
}

#[test]
fn given_the_word_chat_when_check_that_is_valid_then_returns_true() {
    let word = String::from("chat");

    let english_dict_repo: Box<dyn DictionaryRepo> = get_english_dict_repo();
    let valid_word_checker = ValidWordChecker::create(english_dict_repo);

    assert!(valid_word_checker.check_valid_word(&word));
}

#[test]
fn given_the_word_boy_when_check_that_is_valid_then_returns_true() {
    let word = String::from("boy");

    let english_dict_repo: Box<dyn DictionaryRepo> = get_english_dict_repo();
    let valid_word_checker = ValidWordChecker::create(english_dict_repo);

    assert!(valid_word_checker.check_valid_word(&word));
}

#[test]
fn given_the_invalid_word_aboy_when_check_that_is_valid_then_returns_false() {
    let word = String::from("aboy");

    let english_dict_repo: Box<dyn DictionaryRepo> = get_english_dict_repo();
    let valid_word_checker = ValidWordChecker::create(english_dict_repo);
    assert!(!valid_word_checker.check_valid_word(&word));
}

#[test]
fn given_the_invalid_word_coiz_when_check_that_is_valid_then_returns_false() {
    let word = String::from("coiz");
    let english_dict_repo: Box<dyn DictionaryRepo> = get_english_dict_repo();
    let valid_word_checker = ValidWordChecker::create(english_dict_repo);

    assert!(!valid_word_checker.check_valid_word(&word));
}

fn get_english_dict_repo() -> Box<dyn DictionaryRepo> {
    Box::new(JsonDictionaryRepo::create(String::from(
        "./data/words_dictionary.json",
    )))
}
