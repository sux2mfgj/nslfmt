mod lexer;
mod parser;

use parser::parse;

fn main() {
    println!("{:?}", parse(&String::from("declare hello {}")));
}
