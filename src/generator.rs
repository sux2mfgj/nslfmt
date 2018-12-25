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
            let ast = self.parser.next_ast();
            match ast.class {
                ASTClass::EndOfProgram => {
                    return Ok(());
                }
                _ => {
                    let result = ast.generate().iter().fold(
                        "".to_string(),
                        |prev, s| format!("{}\n{}", prev, s));
                    try!(self.writer.write(format!("{}\n", result).as_bytes()));
//                     try!(self.writer.write(format!("{}", ast).as_bytes()));
                }
            }
        }
    }
}
