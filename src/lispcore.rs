use language::*;
use std::rc::Rc;

// ==================== BASIC EVALUATOR ====================

pub fn eval(e: &Cell, a: &Cell) -> Option<Rc<Cell>>{
    if atom(e){
        match e {
            Cell::Symbol(s) => return eval_assoc(e,a),
            _               => return Some(Rc::new(e.clone())),
        };
    } else {
        let car = car(e)?;
        if atom(&car){
            return eval_atom_car(e, &car, a);
        } else {
            return None;
        }
        return None;
    }
    return None;
}

fn eval_assoc(sym: &Cell, a: &Cell) -> Option<Rc<Cell>>{
    let val = assoc(sym, a)?;
    return Some(val)
}

fn eval_atom_car(e :&Cell, macr: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    if is_symbol(macr, Symbol::QUOTE) {
        return cadr(e);
    } else if is_symbol(macr, Symbol::COND) {
        // TODO
        println!("eval cond");
    } 
    return None;
}

fn assoc(x: &Cell, a: &Cell) -> Option<Rc<Cell>> {
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
    if null(x) { // check on the nullness of y?
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

pub fn build_parser_cons(s1: &str,c1: Cell, s2: &str,c2: Cell, s3: &str) -> Cell {
    new_cons(Rc::new(c1), Rc::new(c2))
}

pub fn new_str_with_quotes(mut s: String) -> Cell {
    s.remove(0);
    let l = s.len();
    s.remove(l - 1);
    new_str(s)
}

pub fn build_list(s1: &str, exps : Vec<Cell>, s2: &str, last : Cell) -> Cell {
    let mut last_cdr = last;
    for x in exps.iter().rev() {
        let new_co = new_cons(
            Rc::new(x.clone()), 
            Rc::new(last_cdr)
        ); 
        last_cdr = new_co; 
    }
    return last_cdr;
}

pub fn build_quoted_list(quote: &str,opened_par: &str, exps : Vec<Cell>, closed_par: &str) -> Cell {
    new_cons(
        Rc::new(new_symbol("QUOTE".to_string())), 
        Rc::new(new_cons(
            Rc::new(build_list(opened_par, exps, closed_par, new_nil())),
            Rc::new(new_nil()))))
}

pub fn build_list_with_last_element(opened_par: &str, exps : Vec<Cell>, point: &str, last : Cell, closed_par: &str) -> Cell {
    build_list(opened_par, exps, closed_par, last)
}