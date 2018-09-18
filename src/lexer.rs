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
        while self.tokens.len() == 0 {
            self.supply_tokens();
        }
        if let Some(next_token) = self.tokens.pop_front() {
            if next_token.class == TokenClass::Newline{
                return self.get_next_token();
            }
            else {
                return next_token;
            }
        }
        else {
            panic!("invalid tokens.pop_front()");
        }
    }

    pub fn check_next_token(&mut self) -> Option<&Token>{
        while self.tokens.len() == 0 {
            self.supply_tokens();
        }
        self.tokens.front()
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
                    // TODO
                    '0'...'9' => {
                        self.tokens.push_back(
                                Token::new(Lexer::get_number_token(&mut it), self.line));
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
                    '(' => {
                        self.tokens.push_back(Token::new(
                            TokenClass::Symbol(Symbol::LeftParen),
                            self.line,
                        ));
                        it.next();
                    }
                    ')' => {
                        self.tokens.push_back(Token::new(
                            TokenClass::Symbol(Symbol::RightParen),
                            self.line,
                        ));
                        it.next();
                    }
                    '[' => {
                        self.tokens.push_back(Token::new(
                            TokenClass::Symbol(Symbol::LeftSquareBracket),
                            self.line,
                        ));
                        it.next();
                    }
                    ']' => {
                        self.tokens.push_back(Token::new(
                            TokenClass::Symbol(Symbol::RightSquareBracket),
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
                    ':' => {
                        self.tokens.push_back(Token::new(
                            TokenClass::Symbol(Symbol::Colon),
                            self.line,
                        ));
                        it.next();
                    }
                    ',' => {
                        self.tokens.push_back(Token::new(
                            TokenClass::Symbol(Symbol::Comma),
                            self.line,
                        ));
                        it.next();
                    }
                    '.' => {
                        self.tokens.push_back(
                                Token::new(TokenClass::Symbol(Symbol::Dot), self.line));
                        it.next();
                    }
                    '#' => {
                        self.tokens.push_back(
                                Token::new(
                                    TokenClass::Symbol(Symbol::Sharp),
                                    self.line));
                        it.next();
                    }
                    '"' => {
                        self.tokens.push_back(
                                Token::new(
                                    TokenClass::Symbol(Symbol::DoubleQuote),
                                    self.line));
                        it.next();
                    }
                    ' ' | '\t' => {
                        it.next();
                    }
                    '\n' => {
                        self.tokens.push_back(
                                Token::new(TokenClass::Newline, self.line));
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
            "func_in" => TokenClass::Symbol(Symbol::FuncIn),
            "func_out" => TokenClass::Symbol(Symbol::FuncOut),
            "include" => TokenClass::Macro(Macro::Include),
            "define" => TokenClass::Macro(Macro::Define),
            "undef" => TokenClass::Macro(Macro::Undef),
            "ifdef" => TokenClass::Macro(Macro::Ifdef),
            "ifndef" => TokenClass::Macro(Macro::Ifndef),
            "else"=> TokenClass::Macro(Macro::Else),
            "endif" => TokenClass::Macro(Macro::Endif),
            //TODO
            _ => TokenClass::Identifire(word),
        }
    }
    fn get_number_token<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> TokenClass {
        let mut number = String::new();
        while let Some(&c_next) = iter.peek() {
            /* TODO
             * now, this function can receive the digit value.
             * have to consider hex, oct, bin formats.
             */
            if c_next.is_numeric() | (c_next == '_') {
                number.push_str(&c_next.to_string());
                iter.next();
            } else {
                break;
            }
        }

        TokenClass::Number(number)
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
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 2)
        );
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
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 4)
        );
    }

    #[test]
    fn declare_func_in() {
        let mut b = BufReader::new(File::open("test_code/declare_02.nsl").unwrap());
        let mut l = Lexer::new(&mut b);
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Declare), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("hello_google2".to_string()), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 2)
        );
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
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::FuncIn), 4)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("sugoi".to_string()), 4)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::LeftParen), 4)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("ok".to_string()), 4)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::RightParen), 4)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Semicolon), 4)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 5)
        );
    }

    #[test]
    fn declare_func_out() {
        let mut b = BufReader::new(File::open("test_code/declare_03.nsl").unwrap());
        let mut l = Lexer::new(&mut b);
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Declare), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("hel".to_string()), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 2)
        );
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
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Input), 4)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("ggrks".to_string()), 4)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Semicolon), 4)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Output), 5)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("jk".to_string()), 5)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Semicolon), 5)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::FuncIn), 7)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("sugoi".to_string()), 7)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::LeftParen), 7)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("ok".to_string()), 7)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::RightParen), 7)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Semicolon), 7)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::FuncOut), 8)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("majika".to_string()), 8)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::LeftParen), 8)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("jk".to_string()), 8)
        );

        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::RightParen), 8)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Colon), 8)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("ggrks".to_string()), 8)
        );

        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Semicolon), 8)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 9)
        );
        assert_eq!(l.get_next_token(), Token::new(TokenClass::EndOfProgram, 10));
    }

    #[test]
    fn number() {
        let mut b = "declare ok {input a[12];}".as_bytes();
        let mut l = Lexer::new(&mut b);
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Declare), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("ok".to_string()), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Input), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Identifire("a".to_string()), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::LeftSquareBracket), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Number("12".to_string()), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::RightSquareBracket), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Semicolon), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 1)
        );
    }

    #[test]
    fn declare_04() {
        let mut b = BufReader::new(File::open("test_code/declare_04.nsl").unwrap());
        let mut l = Lexer::new(&mut b);
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::Declare), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Identifire("test".to_string()), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 2));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::Input), 3));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Identifire("aa".to_string()), 3));
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Semicolon), 3)
        );
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::Input), 4));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Identifire("jk".to_string()), 4));
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::Semicolon), 4)
        );
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::FuncIn), 6));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Identifire("ok".to_string()), 6));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::LeftParen), 6));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Identifire("aa".to_string()), 6));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::Comma), 6));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Identifire("jk".to_string()), 6));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::RightParen), 6));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::Semicolon), 6));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 7));
    }

    #[test]
    fn macro_include() {
        let mut b = "#include \"hello.h\"\ndeclare ok {}".as_bytes();
        let mut l = Lexer::new(&mut b);
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Macro(Macro::Include), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::DoubleQuote), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Identifire("hello".to_string()), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::Dot), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Identifire("h".to_string()), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::DoubleQuote), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::Declare), 2));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Identifire("ok".to_string()), 2));
    }

    #[test]
    fn macro_undef() {
        let mut b = "#undef aaaa".as_bytes();
        let mut l = Lexer::new(&mut b);
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Macro(Macro::Undef), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Identifire("aaaa".to_string()), 1));
    }

    #[test]
    fn macro_ifdef() {
        let mut b = "#ifdef aaaa".as_bytes();
        let mut l = Lexer::new(&mut b);
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Macro(Macro::Ifdef), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Identifire("aaaa".to_string()), 1));
    }

    #[test]
    fn macro_ifndef() {
        let mut b = "#ifndef aaaa".as_bytes();
        let mut l = Lexer::new(&mut b);
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Macro(Macro::Ifndef), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Identifire("aaaa".to_string()), 1));
    }

    #[test]
    fn macro_else() {
        let mut b = "#else".as_bytes();
        let mut l = Lexer::new(&mut b);
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Macro(Macro::Else), 1));
    }
    #[test]
    fn macro_endif() {
        let mut b = "#endif".as_bytes();
        let mut l = Lexer::new(&mut b);
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Macro(Macro::Endif), 1));
        assert_eq!(l.get_next_token(), Token::new(TokenClass::EndOfProgram, 1));
    }

    #[test]
    fn macro_define() {
        let mut b = "#define HELLO (12)".as_bytes();
        let mut l = Lexer::new(&mut b);
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Macro(Macro::Define), 1));
        assert_eq!(
                l.get_next_token(),
                Token::new(TokenClass::Identifire("HELLO".to_string()), 1));
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::LeftParen), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Number("12".to_string()), 1)
        );
        assert_eq!(
            l.get_next_token(),
            Token::new(TokenClass::Symbol(Symbol::RightParen), 1)
        );

        assert_eq!(l.get_next_token(), Token::new(TokenClass::EndOfProgram, 1));
    }

}
