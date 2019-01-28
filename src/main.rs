use std::rc::Rc;
extern crate leesp;

use leesp::lispcore::*;

// pub fn new_num<'a>(num: i32) -> Cell<'a> {
//     Cell::Num(num)
// }

// pub fn new_nil<'a>() -> Cell<'a> {
//     Cell::Nil
// }

// pub fn new_cons<'a>(car: &'a Cell, cdr: &'a Cell) -> Cell<'a> {
//     Cell::Cons(car, cdr)
// }

// impl<'a> Display for Cell<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Cell::Num(n) => write!(f, "{}", n),
//             Cell::Nil => write!(f, "Nil"),
//             Cell::Cons(car, cdr) => write!(f, "({} . {})", car, cdr),
//         }
//     }
// }

fn main() {
    let number1 = new_num(1);
    let number2 = new_num(2);
    let symbol1 = new_symbol("n".to_string());
    let n1_rc = Rc::new(number1);
    let s1_rc = Rc::new(symbol1);
    let assoc1 = new_cons(n1_rc, s1_rc);

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

    println!("{}", atom(&number2));

    match car(&assoc1) {
        Some(expr) => println!("{}", atom(&expr)),
        None => println!("shit!"),
    };

    let reference_to_n1 = match car(&assoc1) {
        Some(expr) => expr,
        None => panic!("holy shit"),
    };

    match car(&assoc1) {
        Some(expr) => println!("{}", atom(&expr)),
        None => println!("shit!"),
    };

    println!("{}",  reference_to_n1);
    println!("{}",  assoc1);
    println!("{}",  eq(&assoc1,&assoc1_prime)); // equals!
    println!("{}",  eq(&reference_to_n1,&assoc1_prime)); // false

    match maybe_car(Some(&assoc1)) {
        Some(expr) => expr,
        None => panic!("holy shit"),
    };

    let assoc3 = new_cons(Rc::new(new_nil()), Rc::new(assoc1));
    match caddr(&assoc3) {
        Some(expr) => println!("{}",  expr),
        None => println!("salame!"),
    }
    println!("{}",  assoc3);
}
