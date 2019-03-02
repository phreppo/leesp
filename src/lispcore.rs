use language::*;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

// ==================== BASIC EVALUATOR ====================

pub fn assoc(x: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    match caar(a) {
        Some(reference) => {
            if eq(x, &reference) {
                cdar(a)
            } else {
                match cdr(a) {
                    Some(expr) => assoc(x, &expr),
                    None => None,
                }
            }
        }
        None => None,
    }
}

pub fn pairlis(x: &Cell, y: &Cell, a: Rc<Cell>) -> Option<Rc<Cell>> {
    if null(x) {
        return Some(Rc::clone(&a));
    } else {
        let carx = car(x)?;
        let cary = car(y)?;
        let cdrx = cdr(x)?;
        let cdry = cdr(y)?;
        let rest_of_the_env = pairlis(&cdrx, &cdry, a)?;
        return Some(Rc::new(new_cons(
            Rc::new(new_cons(Rc::clone(&carx), Rc::clone(&cary))),
            Rc::clone(&rest_of_the_env),
        )));
    }
}

pub fn eval(e: &Cell, a: &Cell) -> Option<Rc<Cell>>{
    if atom(e){
        let val = assoc(e, a)?;
        return Some(val);
    }
    return None;
}

pub fn build_parser_cons(s1: &str,c1: Cell, s2: &str,c2: Cell, s3: &str) -> Cell {
    new_cons(Rc::new(c1), Rc::new(c2))
}

pub fn new_str_with_quotes(mut s: String) -> Cell{
    s.remove(0);
    let l = s.len();
    s.remove(l - 1);
    new_str(s)
}