mod token;
mod lexer;

use lexer::Lexer;

fn main() {
    let mut b = "declare hello {input ok; func_in(ok);}".as_bytes();
    let _l = Lexer::new(&mut b);
}
