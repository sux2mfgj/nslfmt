mod ast;
mod generator;
mod lexer;
mod parser;
mod token;

use generator::Generator;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let mut b = "declare hello {input ok; func_in gg (ok);}".as_bytes();
    let mut l = Lexer::new(&mut b);

    let p = Parser::new(&mut l);
    let mut io = std::io::stdout();

    {
        let mut g = Generator::new(p, &mut io);
        g.output_node();
    }
}
