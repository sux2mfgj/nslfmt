mod lexer;
mod token;
mod parser;
mod ast;
mod generator;

use lexer::Lexer;
use parser::Parser;
use generator::Generator;

use std::io::{self, BufWriter, Write};
use std::fs::File;

fn main() {
    let mut b = "declare hello {input ok; func_in(ok);}".as_bytes();
    let mut l = Lexer::new(&mut b);

    let p = Parser::new(&mut l);
    let io = io::stdout();

    //let f = File::open("test_code/fetch.nsl").unwrap();
    //let _io = BufWriter::new(Box::new(f));


    let _g = Generator::new(p, Box::new(io));
}
