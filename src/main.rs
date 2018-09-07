mod lexer;
mod token;
mod parser;
mod ast;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let mut b = "declare hello {input ok; func_in(ok);}".as_bytes();
    let mut l = Lexer::new(&mut b);

    let _p = Parser::new(&mut l);
}
