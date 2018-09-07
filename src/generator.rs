use std::io::{BufWriter, Write};

use parser::*;

pub struct Generator<'a> {
    parser: Parser<'a>,
    writer: Box<Write>,

}

impl<'a> Generator<'a> {
    pub fn new(parser: Parser<'a>, writer: Box<Write>) -> Generator<'a> {
        Generator {
            parser: parser,
            writer: writer,
        }
    }
}

#[cfg(test)]
mod generator_test {
    use super::*;
    use token::*;
    use lexer::*;

    use std::io::{self, BufWriter, Write};
    use std::fs::File;

    #[test]
    fn new_by_stdout() {
        let mut b = "declare hello {input ok; func_in(ok);}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);
        let io = io::stdout();
        let _g = Generator::new(p, Box::new(io));
    }

    #[test]
    fn new_by_file() {
        let mut b = "declare hello {input ok; func_in(ok);}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);
        let f = File::open("test_code/fetch.nsl").unwrap();
        let io = BufWriter::new(f);

        let _g = Generator::new(p, Box::new(io));
    }
}

