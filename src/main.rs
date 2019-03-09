use std::io;
use std::io::prelude::*; 

extern crate leesp;
use leesp::language::*;

fn print(c : Cell) {
    println!("Evaluating {}", c);
    match leesp::evaluator::eval(&c, &minimal_env()){
        Some(result) => println!("{}",  result),
        None => println!("Error evaluating expression")
    }
}

fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match leesp::parser::parse(&input.to_owned()) {
                    Ok(cell) => print(cell),
                    Err(_) => println!("Parse error"),
                }
            }
            Err(error) => println!("Error reading string: {}", error),
        }
    }
}

fn main() {
    repl();
}