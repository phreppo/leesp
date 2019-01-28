use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

const BUILTIN_LAMBDAS: &'static [&'static str] = &["CAR", "CDR", "CONS", "LAMBDA", "QUOTE"];

pub enum Symbol {
    CAR,
    CDR,
    CONS,
    LAMBDA,
    QUOTE,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Cell {
    Nil,
    Num(i32),
    Str(String),
    Symbol(String),
    // qui prima avevo un RC
    Cons(Rc<Cell>, Rc<Cell>),
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

pub fn new_nil() -> Cell {
    Cell::Nil
}

pub fn new_num(num: i32) -> Cell {
    Cell::Num(num)
}

pub fn new_str(str: String) -> Cell {
    Cell::Str(str.to_uppercase())
}

pub fn new_symbol(sym: String) -> Cell {
    Cell::Symbol(sym.to_uppercase())
}

pub fn new_cons(car: Rc<Cell>, cdr: Rc<Cell>) -> Cell {
    Cell::Cons(car, cdr)
}

pub fn atom(cell: &Cell) -> bool {
    match cell {
        Cell::Cons(_, _) => false,
        _ => true,
    }
}

pub fn car(cell: &Cell) -> Option<Rc<Cell>> {
    match cell {
        Cell::Cons(car, _) => Some(Rc::clone(&car)),
        _ => Option::None,
    }
}

pub fn cdr(cell: &Cell) -> Option<Rc<Cell>> {
    match cell {
        Cell::Cons(_, cdr) => Some(Rc::clone(&cdr)),
        _ => Option::None,
    }
}

pub fn maybe_car(maybe_cell: Option<&Cell>) -> Option<Rc<Cell>> {
    match maybe_cell {
        Some(cell) => car(cell),
        None => None,
    }
}

pub fn maybe_cdr(maybe_cell: Option<&Cell>) -> Option<Rc<Cell>> {
    match maybe_cell {
        Some(cell) => cdr(cell),
        None => None,
    }
}

pub fn caar(cell: &Cell) -> Option<Rc<Cell>> {
    match maybe_car(Some(cell)) {
        Some(rc_to_cons_cell) => car(&rc_to_cons_cell),
        _ => Option::None,
    }
}

pub fn cdar(cell: &Cell) -> Option<Rc<Cell>> {
    match maybe_car(Some(cell)) {
        Some(rc_to_cons_cell) => cdr(&rc_to_cons_cell),
        _ => Option::None,
    }
}

pub fn cadr(cell: &Cell) -> Option<Rc<Cell>> {
    match maybe_cdr(Some(cell)) {
        Some(rc_to_cons_cell) => car(&rc_to_cons_cell),
        _ => Option::None,
    }
}

// TODO: test this!
pub fn caddr(cell: &Cell) -> Option<Rc<Cell>> {
    match maybe_cdr(Some(cell)) {
        Some(rc_to_cons_cell) => cadr(&rc_to_cons_cell),
        _ => Option::None,
    }
}

pub fn eq(cell1: &Cell, cell2: &Cell) -> bool {
    cell1 == cell2
}

// first checks if the cell is a symbol, and then if the symbol is the one on the right
pub fn is_symbol(cell: &Cell, symbol: Symbol) -> bool {
    match cell {
        Cell::Symbol(symbol_string) => match symbol {
            Symbol::CAR => BUILTIN_LAMBDAS[0].eq_ignore_ascii_case(&symbol_string),
            Symbol::CDR => BUILTIN_LAMBDAS[1].eq_ignore_ascii_case(&symbol_string),
            Symbol::CONS => BUILTIN_LAMBDAS[2].eq_ignore_ascii_case(&symbol_string),
            Symbol::LAMBDA => BUILTIN_LAMBDAS[3].eq_ignore_ascii_case(&symbol_string),
            Symbol::QUOTE => BUILTIN_LAMBDAS[4].eq_ignore_ascii_case(&symbol_string),
        },
        _ => false,
    }
}

// ==================== BASIC EVALUATOR ====================

pub fn assoc(x: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    match caar(a) {
        Some(reference) => 
            if (eq(x, &reference)){
                cdar(a)
            } else {
                match cdr(a) {
                    Some(expr) => assoc(x,&expr),
                    None => None,
                }
            } ,
        None => None,
    }
}


