use lispcore::*;

pub enum Token {
    OpenPar,
    ClosedPar,
    Dot,
    Num,
    Str,
    Symbol,
}

pub fn next_token(source_program: String) -> Token {
    Token::Dot
}

pub fn parse(source_program: String) -> Result<Cell, String> {
    Err("unsupported".to_string())
}
