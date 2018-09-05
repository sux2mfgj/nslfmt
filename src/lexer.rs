use std::io::Read;

//use token::Token;

pub struct Lexer<'a> {
    pub line: usize,
    reader: &'a Read,
}

impl <'a>Lexer<'a> {
    pub fn new<T: Read>(reader: &T) -> Lexer {
        Lexer {
            line: 1,
            reader: reader,
        }
    }

    //pub fn read_next_token(self) -> Token {
    //}
}

#[cfg(test)]
mod lexer_test{
    use super::*;
    use std::io::BufReader;
    use std::fs::File;

    #[test]
    fn create_instance_with_string() {
        let b = "declare hello {input ok; func_in(ok);}".as_bytes();
        let _l = Lexer::new(&b);
    }

    #[test]
    fn create_instance_with_file() {
        let f = File::open("test_code/fetch.nsl").unwrap();
        //let f = BufReader::new(File::open("test_code/fetch.nsl").unwrap());
        let _l = Lexer::new(&f);
    }

}
