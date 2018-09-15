use std::io::{BufWriter, Write};

use parser::*;
use ast::*;

pub struct Generator<'a, 'b> {
    parser: Parser<'a>,
    //pub writer: Box<Write>,
    writer: &'b mut Write,

}

impl<'a, 'b> Generator<'a, 'b> {
    //pub fn new(parser: Parser<'a>, writer: Box<Write>) -> Generator<'a> {
    pub fn new(parser: Parser<'a>, writer: &'b mut Write) -> Generator<'a, 'b> {
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
                            format!("    func_in {} (", name).as_bytes());
                    self.func_args(args);
                    self.writer.write(b")");

                    if !result.is_empty() {
                        self.writer.write(format!(": {};\n", result).as_bytes());
                    }
                    else {
                        self.writer.write(b";\n");
                    }
                }
                //TODO
                _ => {
                    return None;
                }
            }
        }
        Some("ok".to_string())
    }

    fn func_args(&mut self, args: &Vec<String>) {
        let mut iter = args.iter().peekable();

        while let Some(&arg) = iter.peek() {
            self.writer.write(format!("{}", arg).as_bytes());
            iter.next();

            if None != iter.peek() {
                self.writer.write(b", ");
            }
        }
    }
}

#[cfg(test)]
mod generator_test {
    use super::*;
    use token::*;
    use lexer::*;

    use std::io::{self, Read, BufWriter, Write, Cursor};
    use std::fs::File;

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
    fn output_declare() {
        let mut b = "declare hello {input ok; func_in hh(ok);}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);

        // 出力を指定する
        let mut io = Cursor::new(Vec::new()); // メモリへ(test用)
        // let mut io = io::stdout();              // 標準出力へ(debug用)
        //let io =                          // Fileへ(実用)
        //      BufWriter::new(
        //          File::open(
        //          "test_code/declare.nsl").unwrap());


        //実行する
        {
            let mut g = Generator::new(p, &mut io);
            while let Some(a) = g.output_node() {}
        }

        println!("{:?}", io.position());
        // // テストの際の比較をする
        // let ans = b"declare hello\n{\n    input ok;\n    func_in hh(ok);\n}".as_bytes();
        // let mut out = Vec::new();
        // g.writer.read_to_end(&mut out).unwrap();
        // io.read_to_end(&mut out).unwrap();
        // println!("{:?}", out);
    }
}

