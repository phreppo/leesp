use language::*;
use std::rc::Rc;

// ==================== BASIC EVALUATOR ====================

pub fn eval(e: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    if is_atomic(e) {
        return eval_atom(e, a);
    } else {
        let car = car(e)?;
        if is_atomic(&car) {
            return eval_atom_car(e, &car, a);
        } else {
            return eval_non_atom_car(e, &car, a);
        }
    }
}

fn eval_atom(e: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    match e {
        Cell::Symbol(_) => return eval_assoc(e, a),
        Cell::Nil => return Some(Rc::new(new_nil())),
        _ => return Some(Rc::new(e.clone())), // Num or Str: full functional implementation needs to copy them!
    };
}

fn eval_assoc(sym: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    if is_t(sym) {
        return Some(Rc::new(new_t()));
    }
    return assoc(sym, a);
}

fn assoc(x: &Cell, a: &Cell) -> Option<Rc<Cell>> {
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

fn eval_atom_car(e: &Cell, f: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    if is_symbol(f, Symbol::QUOTE) {
        return cadr(e);
    } else if is_symbol(f, Symbol::COND) {
        let cdre = cdr(e)?;
        return evcon(&cdre, a);
    } else if is_symbol(f, Symbol::LAMBDA) {
        // lambda autoquote feature
        return Some(Rc::new(e.clone()));
    }
    let args = cdr(e)?;
    let evaluated_args = evlis(&args, a)?;
    return apply(f, &evaluated_args, a);
}

fn eval_non_atom_car(e: &Cell, f: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    let args = cdr(e)?;
    let evaluated_args = evlis(&args, a)?;
    return apply(f, &evaluated_args, a);
}

pub fn evlis(m: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    if null(m) {
        return Some(Rc::new(new_nil()));
    } else {
        let first = car(m)?;
        let result = eval(&first, a)?;
        let rest = cdr(m)?;
        let rest_result = evlis(&rest, a)?;
        return Some(Rc::new(new_cons(result, rest_result)));
    }
}

fn apply(f: &Cell, x: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    if is_atomic(f) {
        return apply_atom_f(f, x, a);
    } else {
        return apply_non_atom_f(f, x, a);
    }
}

fn apply_atom_f(f: &Cell, x: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    if is_symbol(f, Symbol::CAR) {
        return caar(x);
    } else if is_symbol(f, Symbol::CDR) {
        return cdar(x);
    } else if is_symbol(f, Symbol::CONS) {
        let car = car(x)?;
        let cadr = cadr(x)?;
        return Some(Rc::new(new_cons(car, cadr)));
    } else if is_symbol(f, Symbol::ATOM) {
        let car = car(x)?;
        return Some(Rc::new(atom(&car)));
    } else if is_symbol(f, Symbol::EQ) {
        let first = car(x)?;
        let second = cadr(x)?;
        return Some(Rc::new(eq(&first, &second)));
    } else if is_symbol(f, Symbol::PLUS) {
        return apply_plus(x, a);
    } else {
        let valued_f = eval(f, a)?;
        return apply(&valued_f, x, a);
    }
}

fn apply_non_atom_f(f: &Cell, x: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    let carf = car(f)?;
    if is_symbol(&carf, Symbol::LAMBDA) {
        let lambda_body = caddr(f)?;
        let cadrf = cadr(f)?;
        let new_env = pairlis(&cadrf, x, Rc::new(a.clone()))?;
        return eval(&lambda_body, &new_env);
    } else if is_symbol(&carf, Symbol::LABEL) {
        // ((label ff (lambda (x) (cond ((atom x) x ) (t (ff (car x)))))) '((a)))
        let caddrf = caddr(f)?;
        let cadrf = cadr(f)?;
        let first_cons = new_cons(cadrf.clone(), caddrf.clone());
        let second_cons = new_cons(Rc::new(first_cons), Rc::new(a.clone()));
        return apply(&caddrf, x, &second_cons);
    }

    // higer order support
    let valued_f = eval(f, a)?;
    return apply(&valued_f, x, a);
}

fn apply_plus(args: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    return apply_plus_rec(args, a, 0);
}

fn apply_plus_rec(args: &Cell, a: &Cell, mut counter: i32) -> Option<Rc<Cell>> {
    if null(args) {
        return Some(Rc::new(new_num(counter)));
    } else {
        let act_var = car(args)?;
        match *act_var {
            Cell::Num(n) => counter += n,
            _ => return None,
        };
        let next_arg = cdr(args)?;
        return apply_plus_rec(&next_arg, a, counter);
    }
}

fn evcon(c: &Cell, a: &Cell) -> Option<Rc<Cell>> {
    let caarc = caar(c)?;
    let case_result = eval(&caarc, a)?;
    if !null(&case_result) {
        let cadarc = cadar(c)?;
        return eval(&cadarc, a);
    } else {
        let cdrc = cdr(c)?;
        return evcon(&cdrc, a);
    }
}

pub fn pairlis(x: &Cell, y: &Cell, a: Rc<Cell>) -> Option<Rc<Cell>> {
    if null(x) || null(y) {
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
