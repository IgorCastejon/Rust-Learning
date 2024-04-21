mod fizzbuzz_test;

#[derive(Debug, PartialEq)]
pub enum FizzBuzzError {
    ZeroInputNotAllowed(String),
}

#[derive(Debug, PartialEq)]
pub struct FizzBuzzInput {
    value: u32,
}

impl FizzBuzzInput {
    pub fn from(value: u32) -> Result<FizzBuzzInput, FizzBuzzError> {
        match value {
            0 => Err(FizzBuzzError::ZeroInputNotAllowed(String::from("0 is not allowed."))),
            _ => Ok(FizzBuzzInput { value }),
        }
    }
}

pub fn fizzbuzz(input: FizzBuzzInput) -> String {
    let input_value = input.value;
    let is_input_divisible_by3 = input_value % 3 == 0;
    let is_input_divisible_by5: bool = input_value % 5 == 0;

    if is_input_divisible_by3 && is_input_divisible_by5 {
        return String::from("fizzbuzz");
    } else if is_input_divisible_by3 {
        return String::from("fizz");
    } else if is_input_divisible_by5 {
        return String::from("buzz");
    }
    input_value.to_string()
}
