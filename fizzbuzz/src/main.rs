use fizzbuzz::{fizzbuzz, FizzBuzzInput};

pub mod fizzbuzz;

fn main() {
    let value: u32 = 0;
    let fizzbuzz_input: Result<FizzBuzzInput, fizzbuzz::FizzBuzzError> = FizzBuzzInput::from(value);
    match fizzbuzz_input {
        Ok(input) => println!("{}", fizzbuzz(input)),
        Err(fizzbuzz::FizzBuzzError::ZeroInputNotAllowed(str)) => println!("{}", str),
    }
}
