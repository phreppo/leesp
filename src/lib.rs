pub mod lispcore;
pub mod parser;

#[cfg(test)]
mod tests {
    use lispcore::*;
    use std::rc::Rc;

    fn n(num: i32) -> Cell {
        return new_num(num);
    }

    fn sy(strng: String) -> Cell {
        return new_symbol(strng);
    }

    fn str(strng: String) -> Cell {
        return new_str(strng);
    }

    fn assoc1() -> Cell {
        let s1_rc = Rc::new(sy("s1".to_string()));
        let n1_rc = Rc::new(n(1));
        return new_cons(s1_rc, n1_rc);
    }

    fn assoc2() -> Cell {
        let s1_rc = Rc::new(sy("s2".to_string()));
        let n1_rc = Rc::new(str("stringone".to_string()));
        return new_cons(s1_rc, n1_rc);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn create_cells() {
    //     let number1 = new_num(1);
    //     let symbol1 = new_symbol("n".to_string());
    //     let assoc1 = new_cons(&symbol1, &number1);
    //     println!("{}", assoc1);
    // }
}
