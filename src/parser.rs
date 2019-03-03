use language::*;
use _parser::*;
use std::rc::Rc;

pub fn parse(s: &str) -> Result<Cell, lalrpop_util::ParseError<usize,Token,&str>> {
    SexprParser::new().parse(s)
}

fn assert_parse_ok (s: &str, c: Cell) {
    match parse(s) {
        Ok(cell) => assert_eq!(cell, c),
        Err(_) => panic!(),
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

#[test]
fn lisp_parser() {
    use std::rc::Rc;

    assert_parse_ok("NIL", new_nil());

    assert_parse_ok("2", new_num(2));

    assert_parse_ok("-2", new_num(-2));

    assert_parse_ok("car", new_symbol("car".to_string()));

    assert_parse_ok("car", new_symbol("CAR".to_string()));

    assert_parse_ok("sTrAnGeVaR123", new_symbol("STRANGEVAR123".to_string()));

    assert_parse_ok("'string'", new_str("string".to_string()));

    assert_parse_ok("(ciao . NIL)", new_cons(Rc::new(new_symbol("ciao".to_string())), Rc::new(new_nil())));

    assert_parse_ok("(ciao . (1 . NIL))",         new_cons(
            Rc::new(new_symbol("ciao".to_string())),
            Rc::new(new_cons(Rc::new(new_num(1)), Rc::new(new_nil())))
    ));
}