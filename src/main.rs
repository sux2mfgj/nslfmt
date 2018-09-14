mod lexer;
mod token;
mod parser;
mod ast;
mod generator;

use lexer::Lexer;
use parser::Parser;
use generator::Generator;

use std::io::{self, BufWriter, Write, Cursor, Read};
use std::fs::File;

fn main() {
    let mut b = "declare hello {input ok; func_in gg (ok);}".as_bytes();
    let mut l = Lexer::new(&mut b);

    let p = Parser::new(&mut l);
    //let io = io::stdout();

    //let f = File::open("test_code/fetch.nsl").unwrap();
    //let _io = BufWriter::new(Box::new(f));
    //let io = Box::new(io::stdout());
    let io = io::stdout();
    //let mut io = Cursor::new(Vec::new());

    {
        let mut g = Generator::new(p, &io);
        g.output_node();
    }

    //let mut out = Vec::new();
    //io.read_to_end(&mut out).unwrap();
    //println!("{:?}", out);
    println!("hello");
}
