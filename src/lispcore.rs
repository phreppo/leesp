use language::*;
use std::rc::Rc;

// ==================== BASIC EVALUATOR ====================

pub fn eval(e: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    if is_atomic(e) {
        println!("eval atom expr {}",  e);
        match e {
            Cell::Symbol(s) => return eval_assoc(e, a),
            Cell::Nil => return Some(Rc::new(new_nil())),
            _ => return Some(Rc::new(e.clone())),
        };
    } else {
        let car = car(e)?;
        if is_atomic(&car) {
            return eval_atom_car(e, &car, a);
        } else {
            return None;
        }
        return None;
    }
    return None;
}

fn eval_assoc(sym: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    let val = assoc(sym, a)?;
    return Some(val);
}

fn eval_atom_car(e: &Cell, f: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    if is_symbol(f, Symbol::QUOTE) {
        return cadr(e);
    } else if is_symbol(f, Symbol::COND) {
        // TODO
        println!("eval cond");
    }
    println!("gonna apply");
    let args = cdr(e)?;
    let evaluated_args = evlis(&args, a)?;
    println!("evlisted: {}", evaluated_args );
    return apply(f, &evaluated_args, a);
}

fn apply(f: &Cell, x: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    println!("appliying apply");
    if is_atomic(f) {
        if is_symbol(f, Symbol::CAR) {
            return caar(x);
        } else if is_symbol(f, Symbol::CDR){
            return cdar(x);
        } else if is_symbol(f, Symbol::CONS) {
            let car = car(x)?;
            let cadr = cadr(x)?;
            return Some(
                Rc::new( new_cons(car, cadr)));
        } else if is_symbol(f, Symbol::ATOM) {
            let car = car(x)?;
            return Some(Rc::new(atom(&car)));
        } else if is_symbol(f, Symbol::EQ) {
            let first = car(x)?;
            let second = cadr(x)?;
            return Some(Rc::new(eq(&first,&second)));
        }
    }
    return None;
}

fn assoc(x: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    // let first_pair = caar(a)
    match caar(a) {
        Some(reference) => {
            if is_eq(x, &reference) {
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
    if null(x) || null(y) {
        // check on the nullness of y?
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

pub fn evlis(m: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    println!("evlis called" );
    if null(m) {
        println!("evlis null" );
        return Some(Rc::new(new_nil()));
    } else {
        let first = car(m)?;
        println!("evlis first: {}", first );
        let result = eval(&first, a)?;
        println!("evlis first valued: {}", result );
        let rest = cdr(m)?;
        let rest_result = evlis(&rest, a)?;
        return Some(Rc::new(new_cons(result, rest_result)));
    }
}
