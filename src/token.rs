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
    Input,              // input
    Output,             // output
    InOut,              // inout
    FuncIn,             // func_in
    FuncOut,            // func_out
                        //TODO
                        // lack some symbols
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
    //Newline,
    EndOfProgram,
}

#[derive(PartialEq, Debug)]
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
