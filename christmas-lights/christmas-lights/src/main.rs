use std::io;

use crate::lights::{Lights, LightsCoordinates};

mod lights;

// Enum to represent the type argument
enum TypeArg {
    TOn,
    TOff,
    Tog,
}

impl TypeArg {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "t-on" => Some(TypeArg::TOn),
            "t-off" => Some(TypeArg::TOff),
            "tog" => Some(TypeArg::Tog),
            _ => None,
        }
    }
}

fn main() {
    let mut lights: Lights = Lights::create();

    loop {
        println!("Enter input in the format 'type x1 y1 x2 y2', or 'exit' to quit:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let trimmed_input = input.trim();
        if trimmed_input == "exit" {
            println!("Exiting the loop.");
            break;
        }

        let parts: Vec<&str> = trimmed_input.split_whitespace().collect();

        if parts.len() != 5 {
            println!("Invalid input format. Please try again.");
            continue;
        }

        let type_arg = match TypeArg::from_str(parts[0]) {
            Some(t) => t,
            None => {
                println!("Invalid type argument: {}", parts[0]);
                continue;
            }
        };

        let x1: u32 = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid x1 value: {}", parts[1]);
                continue;
            }
        };

        let y1: u32 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid y1 value: {}", parts[2]);
                continue;
            }
        };

        let x2: u32 = match parts[3].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid x2 value: {}", parts[3]);
                continue;
            }
        };

        let y2: u32 = match parts[4].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid y2 value: {}", parts[4]);
                continue;
            }
        };

        let first_coord = &LightsCoordinates::create(x1, y1);
        let second_coord = &LightsCoordinates::create(x2, y2);
        lights = match type_arg {
            TypeArg::TOn => lights.turn_on(first_coord, second_coord),
            TypeArg::TOff => lights.turn_off(first_coord, second_coord),
            TypeArg::Tog => lights.toggle(first_coord, second_coord),
        };
        println!("{}", lights.get_number_of_turned_on_lights());
    }
}
