#[cfg(test)]
use crate::word_chains;
use crate::word_chains::{WordChains, WordChainsError};

#[test]
fn given_the_word_cat_and_the_word_cat_when_get_chain_then_should_return_empty_list() {
    let start_word = String::from("cat");
    let end_word = String::from("cat");

    let word_chains: WordChains = WordChains::create();
    let path = word_chains.chain(start_word, end_word);
    assert!(path.unwrap().is_empty());
}

#[test]
fn given_the_word_cat_and_the_word_dog_when_get_chain_then_should_return_list_with_dat_and_dot() {
    let start_word = String::from("cat");
    let end_word = String::from("dog");

    let word_chains: WordChains = WordChains::create();
    let path = word_chains.chain(start_word, end_word);
    assert_eq!(
        vec![String::from("dat"), String::from("dot")],
        path.unwrap()
    );
}

#[test]
fn given_the_word_dog_and_the_word_cat_when_get_chain_then_should_return_list_with_cog_and_cag() {
    let start_word = String::from("dog");
    let end_word = String::from("cat");

    let word_chains: WordChains = WordChains::create();
    let path = word_chains.chain(start_word, end_word);
    assert_eq!(
        vec![String::from("cog"), String::from("cag")],
        path.unwrap()
    );
}

#[test]
fn given_the_word_cat_and_the_word_cot_when_get_chain_then_should_return_empty_list() {
    let start_word = String::from("cat");
    let end_word = String::from("cot");

    let word_chains: WordChains = WordChains::create();
    let path = word_chains.chain(start_word, end_word);
    let expected_path: Vec<String> = Vec::new();
    assert_eq!(expected_path, path.unwrap());
}

#[test]
fn given_the_word_cat_and_the_word_cog_when_get_chain_then_should_return_list_with_cot() {
    let start_word = String::from("cat");
    let end_word = String::from("cog");

    let word_chains: WordChains = WordChains::create();
    let path = word_chains.chain(start_word, end_word);
    let expected_path: Vec<String> = Vec::from([String::from("cot")]);
    assert_eq!(expected_path, path.unwrap());
}

#[test]
fn given_the_word_lead_and_the_word_goad_when_get_chain_then_should_return_list_with_load_and_goad()
{
    let start_word = String::from("lead");
    let end_word = String::from("gold");

    let word_chains: WordChains = WordChains::create();
    let path = word_chains.chain(start_word, end_word);
    assert_eq!(
        vec![String::from("load"), String::from("goad")],
        path.unwrap()
    );
}

#[test]
fn given_the_word_gold_and_the_word_lead_when_get_chain_then_should_return_list_with_goad_and_load()
{
    let start_word = String::from("gold");
    let end_word = String::from("lead");

    let word_chains: WordChains = WordChains::create();
    let path = word_chains.chain(start_word, end_word);
    assert_eq!(
        vec![String::from("goad"), String::from("load")],
        path.unwrap()
    );
}

#[test]
fn given_an_invalid_word_aboy_and_the_word_lead_when_get_chain_then_should_return_error_that_first_word_is_invalid(
) {
    let start_word = String::from("aboy");
    let end_word = String::from("lead");

    let word_chains: WordChains = WordChains::create();
    let path: Result<Vec<String>, WordChainsError> = word_chains.chain(start_word, end_word);
    assert_eq!(WordChainsError::InvalidStartWord, path.unwrap_err());
}

#[test]
fn given_a_valid_word_lead_and_an_invalid_word_aboy_when_get_chain_then_should_return_error_that_end_word_are_invalid(
) {
    let start_word = String::from("lead");
    let end_word = String::from("aboy");

    let word_chains: WordChains = WordChains::create();
    let path: Result<Vec<String>, WordChainsError> = word_chains.chain(start_word, end_word);
    assert_eq!(WordChainsError::InvalidEndWord, path.unwrap_err());
}

#[test]
fn given_an_invalid_word_aboy_and_an_invalid_word_aboy_when_get_chain_then_should_return_error_that_start_and_end_word_are_invalid(
) {
    let start_word = String::from("aboy");
    let end_word = String::from("aboy");

    let word_chains: WordChains = WordChains::create();
    let path: Result<Vec<String>, WordChainsError> = word_chains.chain(start_word, end_word);
    assert_eq!(WordChainsError::InvalidStartAndEndWords, path.unwrap_err());
}

#[test]
fn given_a_valid_uppercase_word_gold_and_a_valid_uppercase_word_lead_when_get_chain_then_should_return_goad_and_load(
) {
    let start_word = String::from("GOLD");
    let end_word = String::from("LEAD");

    let word_chains: WordChains = WordChains::create();
    let path: Result<Vec<String>, WordChainsError> = word_chains.chain(start_word, end_word);
    assert_eq!(
        vec![String::from("goad"), String::from("load")],
        path.unwrap()
    );
}

#[test]
fn given_a_valid_word_gold_and_a_valid_word_boy_when_get_chain_then_should_return_error_as_words_are_not_same_size(
) {
    let start_word = String::from("gold");
    let end_word = String::from("boy");

    let word_chains: WordChains = WordChains::create();
    let path: Result<Vec<String>, WordChainsError> = word_chains.chain(start_word, end_word);
    assert_eq!(WordChainsError::WordsShouldHaveSameSize, path.unwrap_err());
}

#[test]
fn given_a_valid_word_boy_and_a_valid_word_gold_when_get_chain_then_should_return_error_as_words_are_not_same_size(
) {
    let start_word = String::from("boy");
    let end_word = String::from("gold");

    let word_chains: WordChains = WordChains::create();
    let path: Result<Vec<String>, WordChainsError> = word_chains.chain(start_word, end_word);
    assert_eq!(WordChainsError::WordsShouldHaveSameSize, path.unwrap_err());
}

#[test]
fn given_the_word_ruby_and_the_word_code_when_get_chain_then_should_return_rudy_rude_rode() {
    let start_word = String::from("ruby");
    let end_word = String::from("code");

    let word_chains: WordChains = WordChains::create();
    let path: Result<Vec<String>, WordChainsError> = word_chains.chain(start_word, end_word);
    assert_eq!(
        vec![
            String::from("rudy"),
            String::from("rude"),
            String::from("rode"),
        ],
        path.unwrap()
    );
}
