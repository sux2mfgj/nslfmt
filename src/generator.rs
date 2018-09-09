use std::io::{BufWriter, Write};

use parser::*;
use ast::*;

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

    pub fn output_node(&mut self) -> Option<String> {
        let ast = self.parser.next_ast().unwrap();

        match ast.class {
            ASTClass::Declare(id, io_vec, func_vec) => {
                self.writer.write(format!("declare {} {{\n", id).as_bytes());
                self.io_and_func(io_vec);
                self.io_and_func(func_vec);
                self.writer.write(b"}\n");
                self.writer.flush();
                return Some(id);
            }
            ASTClass::EndOfProgram => {
                return None;
            }
            _ => {
                return None;
            }
        }
    }

    fn io_and_func(&mut self, io_vec: Vec<ASTNode>) -> Option<String> {
        let mut iter = io_vec.iter();
        while let Some(node) = iter.next() {
            match node.class {
                ASTClass::Input(ref name, ref bits) => {
                    if(bits == "1")
                    {
                        self.writer.write(
                                format!("    input {};\n", name).as_bytes());
                    } else {
                        self.writer.write(
                                format!("    input {}[{}];\n", name, bits).as_bytes());
                    }
                }
                ASTClass::FuncIn(ref name, ref args, ref result) => {
                    self.writer.write(
                            format!("    func_in {}", name).as_bytes());
                }
                //TODO
                _ => {
                    return None;
                }
            }
        }
        Some("ok".to_string())
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
        let mut b = "declare hello {input ok; func_in hh (ok);}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);
        let io = io::stdout();
        let _g = Generator::new(p, Box::new(io));
    }

    #[test]
    fn new_by_file() {
        let mut b = "declare hello {input ok; func_in hh(ok);}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);
        let f = File::open("test_code/fetch.nsl").unwrap();
        let io = BufWriter::new(f);

        let _g = Generator::new(p, Box::new(io));
    }

    #[test]
    fn output_declare() {
        let mut b = "declare hello {input ok; func_in hh(ok);}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);
        let io = io::stdout();

        let mut g = Generator::new(p, Box::new(io));
        while let Some(a) = g.output_node() {}
    }
}

