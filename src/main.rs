use std::io;
use std::io::prelude::*; 

extern crate leesp;
use leesp::language::*;

fn eval_cell(c : Cell) {
    println!("Evaluating {}", c);
    match leesp::lispcore::eval(&c, &minimal_env()){
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
                    Ok(cell) => eval_cell(cell),
                    Err(_) => println!("Parse error"),
                }
            }
            Err(error) => println!("error reading string: {}", error),
        }
    }
}

fn main() {
    repl();
}