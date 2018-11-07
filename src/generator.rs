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
        let mut double_newline_flag = false;
        loop {
            let ast = self.parser.next_ast();
            match ast.class {
                ASTClass::Newline => {
                    if double_newline_flag {
                        try!(self.writer.write(format!("\n").as_bytes()));
                        double_newline_flag = false;
                    } else {
                        double_newline_flag = true;
                        continue;
                    }
                }
                ASTClass::EndOfProgram => {
                    return Ok(());
                }
                _ => {
                    double_newline_flag = false;
                    try!(self.writer.write(format!("{}", ast).as_bytes()));
                }
            }
        }
    }
}
