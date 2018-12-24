use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Module,             // module
    Declare,            // declare
    Struct,             // struct
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
    Equal,              // =
    RegAssign,          // :=
    Input,              // input
    Output,             // output
    InOut,              // inout
    FuncIn,             // func_in
    FuncOut,            // func_out
    FuncSelf,
    Func,  // func
    Sharp, // #
    //DoubleQuote, // "
    SingleQuote, // '
    Wire,        // wire
    Reg,         // reg
    ProcName,    // proc_name
    StateName,   // state_name
    Mem,         // mem
    Return,      // return
    Any,         // any
    Alt,         // alt
    Else,        // eles
    State,       // state
    Proc,        // proc
    If,          // if
    For,         // for
    While,       // while
    Seq,         // seq
    Variable,    // variable
    Integer,     // integer
    Generate,    // generate
    Invoke,      // invoke
    Finish,      // finish
    Goto,        // goto
                 // TODO
                 // lack some symbols
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus, // +
    Minus,      // -
    Asterisk, // *
    Slash,    // /
    //ShiftLeft,  // <<
    //ShiftRight, // >>
    And,      // &
    Pipe,     // |
    LogicAnd, // &&
    //LogicOr,    // ||
    //Hat,        // ^
    //Tilde,      // ~
    Equal,       // ==
    GreaterEq,   // >=
    LessEq,      // <=
    GreaterThan, // >
    LessThan,    // <
}

#[derive(Debug, Clone, PartialEq)]
pub enum  UnaryOperator {
    Increment,  // ++
    Decrement,  // --
    Not,        // !
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Plus => {
                write!(f, "+")
            }
            Operator::Minus => {
                write!(f, "-")
            }
            Operator::Asterisk => {
                write!(f, "*")
            }
            Operator::Slash => {
                write!(f, "/")
            }
            Operator::And => {
                write!(f, "&")
            }
            Operator::Pipe => {
                write!(f, "|")
            }
            Operator::LogicAnd=> {
                write!(f, "&")
            }
            Operator::Equal => {
                write!(f, "==")
            }
            Operator::LessEq => {
                write!(f, "<=")
            }
            Operator::GreaterEq => {
                write!(f, ">=")
            }
            Operator::LessThan => {
                write!(f, "<")
            }
            Operator::GreaterThan => {
                write!(f, ">")
            }
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOperator::Not => {
                write!(f, "!")
            }
            UnaryOperator::Increment => {
                write!(f, "++")
            }
            UnaryOperator::Decrement => {
                write!(f, "--")
            }
            _ => {
                panic!("not yet implemented");
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
    //TODO
    //     Else,    // #eles
    Endif, // #endif
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
    UnaryOperator(UnaryOperator),
    Macro(Macro),
    CPPStyleComment(String),
    CStyleComment(Vec<String>),
    Newline,
    EndOfProgram,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub class: TokenClass,
    pub line: usize,
}

impl From<(TokenClass, usize)> for Token {
    fn from(s: (TokenClass, usize)) -> Token {
        Token::new(s.0, s.1)
    }
}

impl From<(Symbol, usize)> for Token {
    fn from(s: (Symbol, usize)) -> Token {
        Token::new(TokenClass::Symbol(s.0), s.1)
    }
}

impl From<(Operator, usize)> for Token {
    fn from(s: (Operator, usize)) -> Token {
        Token::new(TokenClass::Operator(s.0), s.1)
    }
}

impl From<(UnaryOperator, usize)> for Token {
    fn from(s: (UnaryOperator, usize)) -> Token {
        Token::new(TokenClass::UnaryOperator(s.0), s.1)
    }
}

impl From<(Macro, usize)> for Token {
    fn from(s: (Macro, usize)) -> Token {
        Token::new(TokenClass::Macro(s.0), s.1)
    }
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
            TokenClass::String(ref st) => {
                return write!(f, "\"{}\"", st);
            }
            TokenClass::Symbol(Symbol::Input) => {
                return write!(f, "input ");
            }
            TokenClass::Symbol(Symbol::Output) => {
                return write!(f, "output ");
            }
            TokenClass::Symbol(Symbol::FuncOut) => {
                return write!(f, "func_out ");
            }
            TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                return write!(f, "[ ");
            }
            TokenClass::Symbol(Symbol::RightSquareBracket) => {
                return write!(f, " ]");
            }
            TokenClass::Symbol(Symbol::LeftParen) => {
                return write!(f, "( ");
            }
            TokenClass::Symbol(Symbol::RightParen) => {
                return write!(f, " )");
            }
            TokenClass::Symbol(Symbol::Semicolon) => {
                return write!(f, "; ");
            }
            TokenClass::Operator(ref op) => {
                write!(f, " {} ", op)
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
