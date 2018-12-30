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
    let symbol1 = new_symbol("n".to_string());
    let n1_rc = Rc::new(number1);
    let s1_rc = Rc::new(symbol1);
    let assoc1 = new_cons(n1_rc, s1_rc);
    println!("{}", assoc1);
    {
        let reftonum = match assoc1 {
            Cell::Cons(car, cdr) => Rc::clone(&car),
            _ => panic!("fock!"),
        };
        println!("{}", reftonum);
    }
}
