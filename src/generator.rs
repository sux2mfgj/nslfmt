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
                    let mut block_content = ast.generate();
                    let head = block_content.pop_front().unwrap();
                    let result = block_content.iter().fold(
                        head, |prev, s| format!("{}\n{}", prev, s));
                    try!(self.writer.write(format!("{}\n", result).as_bytes()));
                }
            }
        }
    }
}
