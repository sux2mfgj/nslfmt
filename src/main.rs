mod ast;
mod generator;
mod lexer;
mod parser;
mod token;

use std::fs::File;
use std::io::BufReader;

use generator::Generator;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let mut b = BufReader::new(File::open("test_code/declare_03.nsl").unwrap());
    let mut l = Lexer::new(&mut b);

    let mut p = Parser::new(&mut l);
    //let mut io = std::io::stdout();

    println!("{:?}", p.next_ast().unwrap());
    /*
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node();
    }
    */
}
