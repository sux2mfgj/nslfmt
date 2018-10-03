use std::io::{self, Write};

use ast::*;
use parser::*;

pub struct Generator<'a, 'b> {
    parser: Parser<'a>,
    writer: &'b mut Write,
}

impl<'a, 'b> Generator<'a, 'b> {
    pub fn new(parser: Parser<'a>, writer: &'b mut Write) -> Generator<'a, 'b> {
        Generator {
            parser: parser,
            writer: writer,
        }
    }

    pub fn output_node(&mut self) -> Result<(), io::Error> {
        loop {
            let ast = self.parser.next_ast().unwrap();
            if ast.class != ASTClass::EndOfProgram {
                try!(self.writer.write(format!("{}", ast).as_bytes()));
            } else {
                return Ok(());
            }
        }
    }
}
