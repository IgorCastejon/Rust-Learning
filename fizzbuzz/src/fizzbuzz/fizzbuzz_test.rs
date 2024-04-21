#[cfg(test)]
use crate::fizzbuzz::*;

use rstest::*;

#[rstest]
#[case(3)]
#[case(6)]
#[case(9)]
fn given_positive_integer_divisible_by3_but_notby5_when_fizzbuzz_then_should_return_fizz(
    #[case] input: u32,
) {
    let input_value: FizzBuzzInput = FizzBuzzInput::from(input).unwrap();
    let result: String = fizzbuzz(input_value);
    assert_eq!("fizz", result);
}

#[rstest]
#[case(5)]
#[case(10)]
fn given_positive_integer_divisible_notby3_but_by5_when_fizzbuzz_then_should_return_buzz(
    #[case] input: u32,
) {
    let input_value: FizzBuzzInput = FizzBuzzInput::from(input).unwrap();
    let result: String = fizzbuzz(input_value);
    assert_eq!("buzz", result);
}

#[rstest]
#[case(15)]
#[case(30)]
fn given_positive_integer_divisible_by3_and_by5_when_fizzbuzz_then_should_return_fizzbuzz(
    #[case] input: u32,
) {
    let input_value: FizzBuzzInput = FizzBuzzInput::from(input).unwrap();
    let result: String = fizzbuzz(input_value);
    assert_eq!("fizzbuzz", result);
}

#[rstest]
#[case(7)]
#[case(16)]
fn given_positive_integer_divisible_notby3_and_notby5_when_fizzbuzz_then_should_return_input_as_string(
    #[case] input: u32,
) {
    let input_value: FizzBuzzInput = FizzBuzzInput::from(input).unwrap();
    let result: String = fizzbuzz(input_value);
    assert_eq!(input.to_string(), result);
}

#[rstest]
#[case(0)]
fn given_zero_integer_when_get_fizzbuzzinput_then_should_return_error(#[case] input: u32) {
    let result: Result<FizzBuzzInput, FizzBuzzError> = FizzBuzzInput::from(input);
    assert_eq!(
        FizzBuzzError::ZeroInputNotAllowed(String::from("0 is not allowed.")),
        result.unwrap_err()
    );
}

#[rstest]
#[case(3)]
#[case(5)]
#[case(10)]
#[case(15)]
fn given_positive_integer_when_get_fizzbuzzinput_then_should_return_valid_input(
    #[case] input: u32,
) {
    let result: Result<FizzBuzzInput, FizzBuzzError> = FizzBuzzInput::from(input);
    assert_eq!(input, result.unwrap().value);
}
