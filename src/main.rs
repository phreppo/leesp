use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum Cell<'a> {
    // Str(String),
    // Sym(String),
    Num(i32),
    Cons(&'a Cell<'a>, &'a Cell<'a>),
    Nil,
}

pub fn new_num<'a>(num: i32) -> Cell<'a> {
    Cell::Num(num)
}

pub fn new_nil<'a>() -> Cell<'a> {
    Cell::Nil
}

pub fn new_cons<'a>(car: &'a Cell, cdr: &'a Cell) -> Cell<'a> {
    Cell::Cons(car, cdr)
}

impl<'a> Display for Cell<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Num(n) => write!(f, "{}", n),
            Cell::Nil => write!(f, "Nil"),
            Cell::Cons(car, cdr) => write!(f, "({} . {})", car, cdr),
        }
    }
}


fn main() {
    println!("Hello, world!");
}
