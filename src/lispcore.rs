use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Cell {
    Nil,
    Num(i32),
    Str(String),
    Symbol(String),
    // qui prima avevo un RC
    Cons(Box<Cell>, Box<Cell>),
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

pub fn new_cons(car: &Cell, cdr: &Cell) -> Cell {
    Cell::Cons(Box::new(car.clone()), Box::new(cdr.clone()))
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