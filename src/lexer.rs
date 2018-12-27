use std::io::BufRead;
use std::iter::Peekable;
use std::vec::IntoIter;

use token::*;

enum CommentState {
    Finished,
    Continue,
}
struct CommentResult(String, CommentState);

//#[derive(Debug, Clone, PartialEq)]
pub struct Lexer<'a> {
    pub line: usize,
    pub current_position: usize,
    reader: &'a mut BufRead,
    line_buffer: String,
    iter: Peekable<IntoIter<char>>,
    next_token: Token,
}

impl<'a> Lexer<'a> {
    pub fn new<T: BufRead>(reader: &'a mut T) -> Lexer<'a> {
        let init_line = 1;
        let init_position = 1;
        let mut lex = Lexer {
            line: init_line,
            current_position: init_position,
            reader: reader,
            line_buffer: "".to_string(),
            iter: ""
                .to_string()
                .chars()
                .collect::<Vec<_>>()
                .into_iter()
                .peekable(),
            next_token: Token::from((TokenClass::Newline, init_line, init_position)),
        };
        lex.next(false);
        lex
    }

    pub fn peek(&mut self, skip_nl: bool) -> Token {
        if skip_nl {
            self.skip_nl();
        }
        self.next_token.clone()
    }

    pub fn next(&mut self, skip_nl: bool) -> Token {
        if skip_nl {
            self.skip_nl();
        }

        let t = self.next_token.clone();
        if TokenClass::Newline != t.class
        {
            self.current_position += 1;
        }
        self.next_token = self.generate_token();
        t
    }

    fn skip_nl(&mut self) {
        while self.next_token.class == TokenClass::Newline {
            self.next_token = self.generate_token();
        }
    }

    /*
     * Some(EndOfProgram) or None
     */
    fn supply_buffer(&mut self) -> Option<Token> {
        if self.iter.peek() == None {
            let mut buf = Vec::<u8>::new();
            match self.reader.read_until(b'\n', &mut buf) {
                Ok(size) => {
                    if size == 0 {
                        return Some(Token::from((TokenClass::EndOfProgram, self.line, self.current_position)));
                    }
                }
                Err(e) => panic!("{}", e),
            }
            self.line_buffer = String::from_utf8(buf).unwrap();
            self.iter = self
                .line_buffer
                .chars()
                .collect::<Vec<_>>()
                .into_iter()
                .peekable();
        }

        None
    }

    fn generate_token(&mut self) -> Token {
        loop {
            if let Some(t) = self.supply_buffer() {
                return t;
            }
            while let Some(&c) = self.iter.peek() {
                match c {
                    'a'...'z' | 'A'...'Z' | '_' => {
                        let t = self.get_token_from_char();
                        return Token::from((t, self.line, self.current_position));
                    }
                    // TODO
                    '0'...'9' => {
                        let t = self.get_number_token();
                        return Token::from((t, self.line, self.current_position));
                    }
                    '{' => {
                        self.iter.next();
                        return Token::from((Symbol::OpeningBrace, self.line, self.current_position));
                    }
                    '}' => {
                        self.iter.next();
                        return Token::from((Symbol::ClosingBrace, self.line, self.current_position));
                    }
                    '(' => {
                        self.iter.next();
                        return Token::from((Symbol::LeftParen, self.line, self.current_position));
                    }
                    ')' => {
                        self.iter.next();
                        return Token::from((Symbol::RightParen, self.line, self.current_position));
                    }
                    '[' => {
                        self.iter.next();
                        return Token::from((Symbol::LeftSquareBracket, self.line, self.current_position));
                    }
                    ']' => {
                        self.iter.next();
                        return Token::from((Symbol::RightSquareBracket, self.line, self.current_position));
                    }
                    ';' => {
                        self.iter.next();
                        return Token::from((Symbol::Semicolon, self.line, self.current_position));
                    }
                    ':' => {
                        self.iter.next();
                        if let Some('=') = self.iter.peek() {
                            self.iter.next();
                            return Token::from((Symbol::RegAssign, self.line, self.current_position));
                        }
                        return Token::from((Symbol::Colon, self.line, self.current_position));
                    }
                    ',' => {
                        self.iter.next();
                        return Token::from((Symbol::Comma, self.line, self.current_position));
                    }
                    '.' => {
                        self.iter.next();
                        return Token::from((Symbol::Dot, self.line, self.current_position));
                    }

                    '#' => {
                        self.iter.next();
                        return Token::from((Symbol::Sharp, self.line, self.current_position));
                    }
                    '*' => {
                        self.iter.next();
                        return Token::from((Operator::Asterisk, self.line, self.current_position));
                    }
                    '+' => {
                        self.iter.next();
                        if let Some(&c_next) = self.iter.peek() {
                            if c_next == '+' {
                                self.iter.next();
                                return Token::from((UnaryOperator::Increment, self.line, self.current_position));
                            }
                        }
                        return Token::from((Operator::Plus, self.line, self.current_position));
                    }
                    '-' => {
                        self.iter.next();
                        if let Some(&c_next) = self.iter.peek() {
                            if c_next == '-' {
                                self.iter.next();
                                return Token::from((UnaryOperator::Decrement, self.line, self.current_position));
                            }
                        }
                        return Token::from((Operator::Minus, self.line, self.current_position));
                    }
                    '|' => {
                        self.iter.next();
                        if let Some(&cc) = self.iter.peek() {
                            if cc == '|' {
                                self.iter.next();
                                return Token::from(
                                    (Operator::LogicOr, self.line, self.current_position));
                            }
                        }
                        return Token::from((Operator::Pipe, self.line, self.current_position));
                    }
                    '^' => {
                        self.iter.next();
                        return Token::from((Operator::Hat, self.line, self.current_position));
                    }
                    '!' => {
                        self.iter.next();
                        if let Some(&cc) = self.iter.peek() {
                            match cc {
                                '=' => {
                                    self.iter.next();
                                    return Token::from((Operator::NotEqual, self.line, self.current_position));
                                }
                                _ => return Token::from((UnaryOperator::Not, self.line, self.current_position)),
                            }
                        }
                    }
                    '\'' => {
                        self.iter.next();
                        return Token::from((Symbol::SingleQuote, self.line, self.current_position));
                    }
                    '=' => {
                        self.iter.next();
                        if let Some(&equal) = self.iter.peek() {
                            match equal {
                                '=' => {
                                    self.iter.next();
                                    return Token::from((Operator::Equal, self.line, self.current_position));
                                }
                                _ => {
                                    return Token::from((Symbol::Equal, self.line, self.current_position));
                                }
                            }
                        }
                    }
                    '>' => {
                        self.iter.next();
                        if let Some(&eq) = self.iter.peek() {
                            match eq {
                                '=' => {
                                    self.iter.next();
                                    return Token::from((Operator::GreaterEq, self.line, self.current_position));
                                }
                                '>' => {
                                    self.iter.next();
                                    return Token::from((Operator::ShiftRight, self.line, self.current_position));
                                }
                                _ => {
                                    return Token::from((
                                        Operator::GreaterThan,
                                        self.line,
                                        self.current_position
                                    ));
                                }
                            }
                        }
                    }
                    '<' => {
                        self.iter.next();
                        if let Some(&eq) = self.iter.peek() {
                            match eq {
                                '=' => {
                                    self.iter.next();
                                    return Token::from((Operator::LessEq, self.line, self.current_position));
                                }
                                '<' => {
                                    self.iter.next();
                                    return Token::from((Operator::ShiftLeft, self.line, self.current_position));
                                }
                                _ => {
                                    return Token::from((Operator::LessThan, self.line, self.current_position));
                                }
                            }
                        }
                    }
                    '&' => {
                        self.iter.next();
                        if let Some(&and) = self.iter.peek() {
                            match and {
                                '&' => {
                                    self.iter.next();
                                    return Token::from((Operator::LogicAnd, self.line, self.current_position));
                                }
                                _ => {
                                    return Token::from((Operator::And, self.line, self.current_position));
                                }
                            }
                        }
                    }
                    '/' => {
                        self.iter.next();
                        if let Some(&slash) = self.iter.peek() {
                            match slash {
                                // single line comment
                                '/' => {
                                    self.iter.next();
                                    let comment = self.get_string_until_newline();
                                    return Token::from((
                                        TokenClass::CPPStyleComment(comment),
                                        self.line,
                                        self.current_position
                                    ));
                                }
                                // multi-line comment
                                '*' => {
                                    self.iter.next();
                                    let comment_list =
                                        self.get_string_for_multiline_comment();
                                    return Token::from((
                                        TokenClass::CStyleComment(comment_list),
                                        self.line,
                                        self.current_position,
                                    ));
                                }
                                _ => return Token::from((Operator::Slash, self.line, self.current_position)),
                            }
                        } else {
                            panic!("panic");
                        }
                    }
                    '"' => {
                        self.iter.next();
                        let mut name = String::new();
                        loop {
                            if let Some(nc) = self.iter.next() {
                                if nc == '"' {
                                    break;
                                } else {
                                    name.push_str(&nc.to_string());
                                }
                            } else {
                                panic!("error");
                            }
                        }
                        return Token::from((TokenClass::String(name), self.line, self.current_position));
                    }
                    '\n' => {
                        let before_line = self.line;
                        self.line += 1;
                        self.iter.next();
                        return Token::from((TokenClass::Newline, before_line, self.current_position));
                    }
                    ' ' | '\t' => {
                        self.iter.next();
                    }
                    _ => {
                        panic!("invalid input {}", c);
                    }
                }
            }
        }
    }

    fn get_token_from_char(&mut self) -> TokenClass {
        let mut word = String::new();
        while let Some(&c_next) = self.iter.peek() {
            if c_next.is_alphanumeric() | (c_next == '_') {
                word.push_str(&c_next.to_string());
                self.iter.next();
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
            "func_self" => TokenClass::Symbol(Symbol::FuncSelf),
            "include" => TokenClass::Macro(Macro::Include),
            "define" => TokenClass::Macro(Macro::Define),
            "undef" => TokenClass::Macro(Macro::Undef),
            "ifdef" => TokenClass::Macro(Macro::Ifdef),
            "ifndef" => TokenClass::Macro(Macro::Ifndef),
            "endif" => TokenClass::Macro(Macro::Endif),
            "wire" => TokenClass::Symbol(Symbol::Wire),
            "reg" => TokenClass::Symbol(Symbol::Reg),
            "proc_name" => TokenClass::Symbol(Symbol::ProcName),
            "state_name" => TokenClass::Symbol(Symbol::StateName),
            "mem" => TokenClass::Symbol(Symbol::Mem),
            "func" => TokenClass::Symbol(Symbol::Func),
            "return" => TokenClass::Symbol(Symbol::Return),
            "any" => TokenClass::Symbol(Symbol::Any),
            "alt" => TokenClass::Symbol(Symbol::Alt),
            "seq" => TokenClass::Symbol(Symbol::Seq),
            "else" => TokenClass::Symbol(Symbol::Else),
            "state" => TokenClass::Symbol(Symbol::State),
            "proc" => TokenClass::Symbol(Symbol::Proc),
            "if" => TokenClass::Symbol(Symbol::If),
            "for" => TokenClass::Symbol(Symbol::For),
            "while" => TokenClass::Symbol(Symbol::While),
            "goto" => TokenClass::Symbol(Symbol::Goto),
            "finish" => TokenClass::Symbol(Symbol::Finish),
            "invoke" => TokenClass::Symbol(Symbol::Invoke),
            "generate" => TokenClass::Symbol(Symbol::Generate),
            "integer" => TokenClass::Symbol(Symbol::Integer),
            "variable" => TokenClass::Symbol(Symbol::Variable),
            "struct" => TokenClass::Symbol(Symbol::Struct),
            "simulation" => TokenClass::Symbol(Symbol::Simulation),
            //TODO
            _ => TokenClass::Identifire(word),
        }
    }

    fn get_string_until_newline(&mut self) -> String {
        let mut word = String::new();
        while let Some(&c_next) = self.iter.peek() {
            if c_next == '\n' {
                break;
            } else {
                word.push_str(&c_next.to_string());
                self.iter.next();
            }
        }
        word
    }

    fn get_string_for_multiline_comment(&mut self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        while let Some(r) = self.get_comment_oneline() {
            self.supply_buffer();
            result.push(r.0);
            match r.1 {
                CommentState::Finished => {
                    return result;
                }
                CommentState::Continue => {}
            }
        }
        panic!("comment is not closed but got EOF");
    }

    fn get_number_token(&mut self) -> TokenClass {
        let mut number = self.get_number();
        if let Some(&c) = self.iter.peek() {
            match c {
                '\'' => {
                    self.iter.next();
                    let id = self.get_token_from_char();
                    let t = Token::from((id, self.line, self.current_position));
                    number.push_str(&format!("'{}", t));
                    return TokenClass::Number(number);
                }
                'x' | 'b' => {
                    self.iter.next();
                    number.push_str(&format!("{}{}", c, self.get_number()));
                    return TokenClass::Number(number);
                }
                _ => {
                    return TokenClass::Number(number);
                }
            }
        }
        panic!("cannot get character");
    }

    fn get_comment_oneline(&mut self) -> Option<CommentResult> {
        let mut word = String::new();
        let mut astarisc_flag = false;
        while let Some(&c_next) = self.iter.peek() {
            self.iter.next();
            match c_next {
                '\n' => {
                    //word.push_str(&c_next.to_string());
                    return Some(CommentResult(word, CommentState::Continue));
                }
                '*' => {
                    word.push_str(&c_next.to_string());
                    astarisc_flag = true;
                }
                '/' => {
                    if astarisc_flag {
                        word.pop();
                        return Some(CommentResult(word, CommentState::Finished));
                    }
                    word.push_str(&c_next.to_string());
                }
                _ => {
                    astarisc_flag = false;
                    word.push_str(&c_next.to_string());
                }
            }
        }
        return Some(CommentResult(word, CommentState::Finished));
    }

    fn get_number(&mut self) -> String {
        let mut number = String::new();
        while let Some(&c_next) = self.iter.peek() {
            if c_next.is_digit(16) | (c_next == '_') {
                number.push_str(&c_next.to_string());
                self.iter.next();
            } else {
                break;
            }
        }

        number
    }
}
