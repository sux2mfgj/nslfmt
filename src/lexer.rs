use std::io::Read;
use std::iter::Peekable;
use std::str::{from_utf8, Chars};
use token::Token;

pub struct Lexer<'a> {
    pub line: usize,
    reader: &'a Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(reader: &Read) -> Lexer {
        Lexer {
            line: 1,
            reader: from_utf8(reader).peekable(),
        }
    }

    pub fn read_next_token(self) -> Token {

    }
}

#[cfg(test)]
mod lexer_test{
    use super::*;

    #[test]
    fn create_instance() {
        let b = "declare hello {input ok; func_in(ok);}".as_bytes();
        let _l = Lexer::new(&b);
    }
}
