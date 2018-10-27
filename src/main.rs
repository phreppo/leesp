use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

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

#[derive(Debug, PartialEq)]
pub enum Cell {
    Nil,
    Num(i32),
    Str(String),
    Symbol(String),
    Cons(Rc<Cell>, Rc<Cell>),
}

pub fn new_nil() -> Cell {
    Cell::Nil
}

pub fn new_num(num: i32) -> Cell {
    Cell::Num(num)
}

pub fn new_str(str: String) -> Cell {
    Cell::Str(str)
}

pub fn new_symbol(sym: String) -> Cell {
    Cell::Str(sym)
}

pub fn new_cons(car: Cell, cdr: Cell) -> Cell {
    Cell::Cons(Rc::new(car), Rc::new(cdr))
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Nil => write!(f, "Nil"),
            Cell::Num(n) => write!(f, "{}", n),
            Cell::Str(s) => write!(f, "\"{}\"", s),
            Cell::Symbol(s) => write!(f, "\"{}\"", s),
            Cell::Cons(car, cdr) => write!(f, "({} . {})", car, cdr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn create_cells() {
        let number1 = new_num(1);
        let symbol1 = new_symbol("n".to_string());
        let assoc1 = new_cons(symbol1, number1);
        println!("{}", assoc1);
    }
}

fn main() {
    let number1 = new_num(1);
    let symbol1 = new_symbol("n".to_string());
    let assoc1 = new_cons(symbol1, number1);
    println!("{}", assoc1);
}
