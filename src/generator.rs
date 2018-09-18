use std::io::Write;

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

    pub fn output_node(&mut self) {
        let ast = self.parser.next_ast().unwrap();

        match ast.class {
            ASTClass::Declare(id, interfaces) => {
                self.writer.write(format!("declare {} {{\n", id).as_bytes()).unwrap();
                self.io_and_func(interfaces);
                self.writer.write(b"}\n").unwrap();
                self.writer.flush().unwrap();
            }
            ASTClass::EndOfProgram => {
                return;
            }
            _ => {
                return;
            }
        }
    }

    fn io_and_func(&mut self, io_vec: Vec<ASTNode>) {
        let mut iter = io_vec.iter();
        while let Some(node) = iter.next() {
            match node.class {
                ASTClass::Input(ref name, ref bits) => {
                    if bits == "1" {
                        self.writer
                            .write(format!("    input {};\n", name).as_bytes()).unwrap();
                    } else {
                        self.writer
                            .write(format!("    input {}[{}];\n", name, bits).as_bytes()).unwrap();
                    }
                }
                ASTClass::FuncIn(ref name, ref args, ref result) => {
                    self.writer
                        .write(format!("    func_in {}(", name).as_bytes()).unwrap();
                    self.func_args(args);
                    self.writer.write(b")").unwrap();

                    if !result.is_empty() {
                        self.writer.write(format!(": {};\n", result).as_bytes()).unwrap();
                    } else {
                        self.writer.write(b";\n").unwrap();
                    }
                }
                //TODO
                _ => {
                    return;
                }
            }
        }
    }

    fn func_args(&mut self, args: &Vec<String>) {
        let mut iter = args.iter().peekable();

        while let Some(&arg) = iter.peek() {
            self.writer.write(format!("{}", arg).as_bytes()).unwrap();
            iter.next();

            if None != iter.peek() {
                self.writer.write(b", ").unwrap();
            }
        }
    }
}

#[cfg(test)]
mod generator_test {
    use super::*;
    use lexer::*;
    //use token::*;

    use std::fs::File;
    use std::io::{self, BufWriter, Cursor};

    #[test]
    fn new_by_stdout() {
        let mut b = "declare hello {input ok; func_in hh (ok);}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);
        let mut io = io::stdout();
        let _g = Generator::new(p, &mut io);
    }

    #[test]
    fn new_by_file() {
        let mut b = "declare hello {input ok; func_in hh(ok);}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);
        let f = File::open("test_code/fetch.nsl").unwrap();
        let mut io = BufWriter::new(f);

        let _g = Generator::new(p, &mut io);
    }

    #[test]
    fn output_declare_to_file() {
        let mut b = "declare hello {input ok; func_in hh(ok);}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);

        let mut io = BufWriter::new(File::create("/tmp/out.nsl").unwrap());

        {
            let mut g = Generator::new(p, &mut io);
            /* while let Some(_a) = g.output_node() {} */
            g.output_node();
        }
    }

    #[test]
    fn output_declare_to_stdio() {
        let mut b = "declare hello {input ok; func_in hh(ok);}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);

        let mut io = io::stdout();

        {
            let mut g = Generator::new(p, &mut io);
            /* while let Some(_a) = g.output_node() {} */
            g.output_node();
        }
    }

    #[test]
    fn output_declare_to_cursor() {
        let mut b = "declare hello {input ok; func_in hh(ok);}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);

        let mut io = Cursor::new(Vec::new());
        {
            let mut g = Generator::new(p, &mut io);
            /* while let Some(_a) = g.output_node() {} */
            g.output_node();
        }

        let ans = "declare hello {\n    input ok;\n    func_in hh(ok);\n}\n";
        /* if let  */
        if let Ok(s) = String::from_utf8(io.into_inner()) {
            println!("{}", s);
            assert_eq!(s, ans);
        }
    }

    #[test]
    fn func_args() {
        let mut b = "declare hello {input ok; input test; func_in hh(ok, test);}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);

        let mut io = Cursor::new(Vec::new());
        {
            let mut g = Generator::new(p, &mut io);
            /* while let Some(_a) = g.output_node() {} */
            g.output_node();
        }

        let ans = "declare hello {\n    input ok;\n    input test;\n    func_in hh(ok, test);\n}\n";
        if let Ok(s) = String::from_utf8(io.into_inner()) {
            println!("{}", s);
            assert_eq!(s, ans);
        }
    }
}
