#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Module,
    Declare,
    OpeningBrace,
    ClosingBrace,
    LeftParen,
    RightParen,
    LeftSquareBracket,
    RightSquareBracket,
    Semicolon,
    Colon,
    Comma,
    Input,
    Output,
    InOut,
    FuncIn,
    FuncOut,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenClass {
    Identifire(String),
    /*
     * HDLはいくつかの表現があり、用途によってかき分け、
     * コードフォーマッタではいじらないのでStringで持ちそのまま出す
     */
    Number(String),
    Symbol(Symbol),
    Newline,
}

pub struct Token {
    pub class: TokenClass,
    pub line: usize
}

impl Token {
    pub fn new(class: TokenClass, line: usize) -> Token {
        Token {
            class: class,
            line: line,
        }
    }
}
