use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

const BUILTIN_LAMBDAS: &'static [&'static str] = &[
    "CAR", 
    "CDR", 
    "CONS", 
    "LAMBDA", 
    "QUOTE"
];

#[derive(Debug, PartialEq, Clone)]
pub enum Cell {
    Nil,
    Num(i32),
    Str(String),
    Symbol(String), 
    // qui prima avevo un RC
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

pub fn new_cons(car: Rc<Cell>, cdr: Rc<Cell>) -> Cell {
    Cell::Cons(car, cdr)
}

pub fn atom(cell: &Cell) -> bool{
    match cell {
        Cell::Cons(_,_) => false,
        _ => true,
    }
}

pub fn car(cell: &Cell) -> Option<Rc<Cell>> {
    match cell {
        Cell::Cons(car,_) => Some(Rc::clone(&car)),
        _ => Option::None,
    }
}

pub fn cdr(cell: &Cell) -> Option<Rc<Cell>> {
    match cell {
        Cell::Cons(_,cdr) => Some(Rc::clone(&cdr)),
        _ => Option::None,
    }
}

pub fn eq(cell1: &Cell, cell2: &Cell) -> bool{
    cell1 == cell2
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