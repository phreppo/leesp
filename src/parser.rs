use language::*;
use _parser::*;

pub fn parse(s: &str) -> Cell {
    // TODO: permettere di recuperare i parsing errors
    SexprParser::new().parse(s).unwrap()
}

#[test]
fn lisp_parser() {
    use std::rc::Rc;

    let result = parse("NIL");
    assert_eq!(result, new_nil());

    let result = parse("2");
    assert_eq!(result, new_num(2));

    let result = parse("-2");
    assert_eq!(result, new_num(-2));

    let result = parse("car");
    assert_eq!(result, new_symbol("car".to_string()));

    let result = parse("car");
    assert_eq!(result, new_symbol("CAR".to_string()));

    let result = parse("sTrAnGeVaR123");
    assert_eq!(result, new_symbol("STRANGEVAR123".to_string()));

    let result = parse("'string'");
    assert_eq!(result, new_str("string".to_string()));

    let result = parse("(ciao . NIL)");
    assert_eq!(
        result,
        new_cons(Rc::new(new_symbol("ciao".to_string())), Rc::new(new_nil()))
    );

    let result = parse("(ciao . (1 . NIL))");
    assert_eq!(
        result,
        new_cons(
            Rc::new(new_symbol("ciao".to_string())),
            Rc::new(new_cons(Rc::new(new_num(1)), Rc::new(new_nil())))
        )
    );
}