use language::*;
use _parser::*;

pub fn parse(s: &str) -> Cell {
    // TODO: permettere di recuperare i parsing errors
    SexprParser::new().parse(s).unwrap()
}