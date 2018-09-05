use std::collections::LinkedList;
use std::io::BufRead;
use std::iter::Peekable;

use token::*;

pub struct Lexer<'a> {
    pub line: usize,
    reader: &'a mut BufRead,
    tokens: LinkedList<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new<T: BufRead>(reader: &'a mut T) -> Lexer<'a> {
        Lexer {
            line: 1,
            reader: reader,
            tokens: LinkedList::new(),
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        if self.tokens.len() == 0 {
            self.supply_tokens();
        }
        match self.tokens.pop_front() {
            Some(next_token) => next_token,
            None => {
                panic!("invalid tokens.pop_front()");
            }
        }
    }

    fn supply_tokens(&mut self) {
        let mut buf = Vec::<u8>::new();
        let t = self.reader.read_until(b'\n', &mut buf).unwrap();
        if t == 0 {
            self.tokens
                .push_back(Token::new(TokenClass::EndOfProgram, self.line));
        } else {
            let s = String::from_utf8(buf).expect("from_utf8 failed");
            let mut it = s.chars().peekable();
            while let Some(&c) = it.peek() {
                match c {
                    'a'...'z' | 'A'...'Z' | '_' => {
                        self.tokens.push_back(Token::new(
                            Lexer::get_token_from_char(&mut it),
                            self.line,
                        ));
                    }
                    '{' => {
                        self.tokens.push_back(Token::new(
                            TokenClass::Symbol(Symbol::OpeningBrace),
                            self.line,
                        ));
                        it.next();
                    }
                    '}' => {
                        self.tokens.push_back(Token::new(
                            TokenClass::Symbol(Symbol::ClosingBrace),
                            self.line,
                        ));
                        it.next();
                    }
                    ';' => {
                        self.tokens.push_back(Token::new(
                            TokenClass::Symbol(Symbol::Semicolon),
                            self.line,
                        ));
                        it.next();
                    }
                    ' ' | '\t' => {
                        it.next();
                    }
                    '\n' => {
                        self.tokens
                            .push_back(Token::new(TokenClass::Newline, self.line));
                        self.line += 1;
                        it.next();
                    }
                    _ => {
                        panic!("invalid input");
                    }
                }
            }
        }
    }

    fn get_token_from_char<T: Iterator<Item = char>>(
        iter: &mut Peekable<T>,
    ) -> TokenClass {
        let mut word = String::new();
        while let Some(&c_next) = iter.peek() {
            if c_next.is_alphanumeric() | (c_next == '_') {
                word.push_str(&c_next.to_string());
                iter.next();
            } else {
                break;
            }
        }
        match word.as_str() {
            "declare" => TokenClass::Symbol(Symbol::Declare),
            "module" => TokenClass::Symbol(Symbol::Module),
            "input" => TokenClass::Symbol(Symbol::Input),
            "output" => TokenClass::Symbol(Symbol::Output),
            "inout" => TokenClass::Symbol(Symbol::InOut),
            //TODO
            _ => TokenClass::Identifire(word),
        }
    }
}

#[cfg(test)]
mod lexer_test {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn create_instance_with_string() {
        let mut b = "declare hello {input ok; func_in(ok);}".as_bytes();
        let _l = Lexer::new(&mut b);
    }

    #[test]
    fn create_instance_with_file() {
        let mut f = BufReader::new(File::open("test_code/fetch.nsl").unwrap());
        let _l = Lexer::new(&mut f);
    }

    #[test]
    fn get_token_str() {
        let mut b = "declare".as_bytes();
        let mut l = Lexer::new(&mut b);
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Declare), 1)
        );
    }

    #[test]
    fn get_token_file() {
        let mut f = BufReader::new(File::open("test_code/declare.nsl").unwrap());
        let mut l = Lexer::new(&mut f);
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Declare), 1)
        );
        assert_eq!(l.get_next_token(), Token::new(TokenClass::Newline, 1));
    }

    #[test]
    fn braces_and_newline() {
        let mut b = "declare {  \n }".as_bytes();
        let mut l = Lexer::new(&mut b);
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Declare), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 1)
        );
        assert_eq!(l.get_next_token(), Token::new(TokenClass::Newline, 1));
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 2)
        );
    }

    #[test]
    fn declare_with_input() {
        let mut b = BufReader::new(File::open("test_code/declare_01.nsl").unwrap());
        let mut l = Lexer::new(&mut b);
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Declare), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("hello".to_string()), 1)
        );
        assert_eq!(l.get_next_token(), Token::new(TokenClass::Newline, 1));
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 2)
        );
        assert_eq!(l.get_next_token(), Token::new(TokenClass::Newline, 2));
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Input), 3)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("ok".to_string()), 3)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Semicolon), 3)
        );
        assert_eq!(l.get_next_token(), Token::new(TokenClass::Newline, 3));
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 4)
        );
    }
}
