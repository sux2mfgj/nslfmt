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
                        //SingleQuote,        // '
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
