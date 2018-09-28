use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Module,             // module
    Declare,            // declare
    OpeningBrace,       // {
    ClosingBrace,       // }
    LeftParen,          // (
    RightParen,         // )
    LeftSquareBracket,  // [
    RightSquareBracket, // ]
    Semicolon,          // ;
    Colon,              // :
    Comma,              // ,
    Dot,                // .
    Input,              // input
    Output,             // output
    InOut,              // inout
    FuncIn,             // func_in
    FuncOut,            // func_out
    Sharp,              // #
    DoubleQuote,        // "
    SingleQuote,        // '
                        //TODO
                        // lack some symbols
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,       // +
    Minus,      // -
    Asterisk,   // *
    Slash,      // /
    ShiftLeft,  // <<
    ShiftRight, // >>
    And,        // &
    Pipe,       // |
    Hat,        // ^
    Tilde,      // ~
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Slash => {
                return write!(f, "/");
            }
            _ => {
                panic!("the operator ({}) is not implemented yet", self);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Macro {
    Include, // #include
    Define,  // #define
    Undef,   // #undef
    Ifdef,   // #ifdef
    Ifndef,  // #ifndef
    Else,    // #eles
    Endif,   // #endif
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenClass {
    Identifire(String),
    /*
     * HDLはいくつかの表現があり、用途によってかき分け、
     * コードフォーマッタではいじらないのでStringで持ちそのまま出す
     */
    Number(String),
    String(String),
    // "hello.h" 等
    Symbol(Symbol),
    Operator(Operator),
    Macro(Macro),
    Comment(String),
    Newline,
    EndOfProgram,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub class: TokenClass,
    pub line: usize,
}

impl Token {
    pub fn new(class: TokenClass, line: usize) -> Token {
        Token {
            class: class,
            line: line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.class {
            TokenClass::Identifire(ref id) => {
                return write!(f, "{}", id);
            }
            TokenClass::Number(ref num) => {
                return write!(f, "{}", num);
            }
            TokenClass::Symbol(Symbol::Input) => {
                return write!(f, " input ");
            }
            TokenClass::Symbol(Symbol::Output) => {
                return write!(f, " output ");
            }
            TokenClass::Symbol(Symbol::FuncOut) => {
                return write!(f, " func_out ");
            }
            TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                return write!(f, "[ ");
            }
            TokenClass::Symbol(Symbol::RightSquareBracket) => {
                return write!(f, " ]");
            }
            TokenClass::Symbol(Symbol::LeftParen) => {
                return write!(f, "(( ");
            }
            TokenClass::Symbol(Symbol::RightParen) => {
                return write!(f, " )");
            }
            TokenClass::Symbol(Symbol::Semicolon) => {
                return write!(f, ";");
            }
            TokenClass::Operator(Operator::Slash) => {
                return write!(f, " / ");
            }
            //TODO
            _ => {
                panic!(
                    "For the token {:?}, this function does not implemented yet.",
                    self
                );
            }
        }
    }
}
