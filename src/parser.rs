use language::*;
use _parser::*;

pub fn parse(s: &str) -> Result<Cell, lalrpop_util::ParseError<usize,Token,&str>> {
    SexprParser::new().parse(s)
}

fn assert_parse_ok (s: &str, c: Cell) {
    match parse(s) {
        Ok(cell) => assert_eq!(cell, c),
        Err(_) => panic!(),
    }
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