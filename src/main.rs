use std::io;
use std::rc::Rc;
use std::io::prelude::*; 

extern crate leesp;
use leesp::lispcore::*;
use leesp::language::*;
use leesp::parser::*;

fn eval_cell(c : Cell) {
    println!("Evaluating {}", c);
    match leesp::lispcore::eval(&c, &new_nil()){
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
            Ok(n) => {
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

fn tests() {
    let number1 = new_num(1);
    let number2 = new_num(2);
    let symbol1 = new_symbol("n".to_string());
    let n1_rc = Rc::new(number1);
    let s1_rc = Rc::new(symbol1);
    let assoc1 = new_cons(s1_rc, n1_rc);

    let number1_prime = new_num(1);
    let number2_prime = new_num(2);
    let symbol1_prime = new_symbol("n".to_string());
    let n1_rc_prime = Rc::new(number1_prime);
    let s1_rc_prime = Rc::new(symbol1_prime);
    let assoc1_prime = new_cons(n1_rc_prime, s1_rc_prime);

    println!("{}", assoc1);
    // {
    //     let reftonum = match assoc1 {
    //         Cell::Cons(car, cdr) => Rc::clone(&car),
    //         _ => panic!("fock!"),
    //     };
    //     println!("{}", reftonum);
    // }

    let reference_to_n1 = match car(&assoc1) {
        Some(expr) => expr,
        None => panic!("holy shit"),
    };

    println!("{}", reference_to_n1);
    println!("{}", assoc1);

    match maybe_car(Some(&assoc1)) {
        Some(expr) => expr,
        None => panic!("holy shit"),
    };

    // let assoc3 = new_cons(Rc::new(new_nil()), Rc::new(assoc1));
    // match caddr(&assoc3) {
    //     Some(expr) => println!("{}",  expr),
    //     None => println!("salame!"),
    // }
    // println!("{}",  assoc3);

    let car_cell = new_symbol("QUOTE".to_string());
    println!("> {}", is_symbol(&car_cell, Symbol::CDR));

    let symbol2 = new_symbol("s2".to_string());
    let string2 = new_str("salame".to_string());
    // let assoc2 = new_cons(Rc::new(symbol2), Rc::new(string2));
    // let env1 = new_cons(Rc::new(assoc1), Rc::new(new_cons(Rc::new(assoc2),Rc::new(new_nil()))));
    // println!("{}",  env1);
    // match assoc(&new_symbol("s2".to_string()),&env1) {
    //     Some(expr) => println!("{}",  expr),
    //     None => println!("We have no found any assoc" ),
    // }
    let first_list = new_cons(Rc::new(symbol2), Rc::new(new_nil()));
    println!("{}", first_list);
    let second_list = new_cons(Rc::new(string2), Rc::new(new_nil()));
    println!("{}", second_list);
    let pairlis1 = pairlis(
        &first_list,
        &second_list,
        Rc::new(new_cons(Rc::new(assoc1), Rc::new(new_nil()))),
    );
    match pairlis1 {
        Some(expr) => match eval(&new_symbol("n".to_string()), &expr) {
            Some(result) => println!("{}", result),
            None => println!("Shitet"),
        },
        None => println!("shit"),
    }
}
