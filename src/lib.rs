pub mod parser;
pub mod lispcore;

#[cfg(test)]
mod tests {
    use lispcore::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn create_cells() {
        let number1 = new_num(1);
        let symbol1 = new_symbol("n".to_string());
        let assoc1 = new_cons(&symbol1, &number1);
        println!("{}", assoc1);
    }
}