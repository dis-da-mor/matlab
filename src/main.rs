use std::io::{Write, stdin, stdout};

mod colours;
mod tools;
use colours::println_error;
mod function;
mod matlab;
mod matrix;
mod tokeniser;
use crate::matlab::Evaluator;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();

    let mut evaluator = Evaluator::new();

    loop {
        print!(">> ");
        match stdout.flush() {
            Ok(_) => {}
            Err(_) => {}
        };

        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println_error("Error: Could not read input...".to_owned());
                continue;
            }
        }
        input = input.trim().to_string();

        if &input.to_lowercase() == "quit" {
            return;
        }

        evaluator.evaluate(&input);
    }
}
