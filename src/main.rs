mod ast;
mod generator;
mod lexer;
mod parser;
mod token;

use generator::Generator;
use lexer::Lexer;
use parser::Parser;

use std::fs::File;
use std::io::{self, BufWriter, Cursor, Read, Write};

fn main() {
    let mut b = "declare hello {input ok; func_in gg (ok);}".as_bytes();
    let mut l = Lexer::new(&mut b);

    let p = Parser::new(&mut l);
    //let io = io::stdout();

    //let f = File::open("test_code/fetch.nsl").unwrap();
    //let _io = BufWriter::new(Box::new(f));
    //let io = Box::new(io::stdout());
    let mut io = io::stdout();
    //let mut io = Cursor::new(Vec::new());

    {
        let mut g = Generator::new(p, &mut io);
        g.output_node();
    }

    //let mut out = Vec::new();
    //io.read_to_end(&mut out).unwrap();
    //println!("{:?}", out);
    println!("hello");
}
