use std::io::{Error, Write};

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
        loop {
            let ast = self.parser.next_ast().unwrap();
            if ast.class != ASTClass::EndOfProgram {
                self.writer.write(format!("{}", ast).as_bytes());
            }
            else {
                return;
            }
        }
    }
}

#[cfg(test)]
mod generator_test {
    use super::*;
    use lexer::*;

    use std::fs::File;
    use std::io::{self, BufReader, BufWriter, Cursor};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static call_count: AtomicUsize = AtomicUsize::new(0);
    fn get_value_with_lock() -> usize {
        return call_count.fetch_add(1, Ordering::Relaxed);
    }

    #[test]
    fn new_by_stdout() {
        let mut b = "declare hello {}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);
        let mut io = Cursor::new(Vec::new());
        {
            let mut g = Generator::new(p, &mut io);
            g.output_node();
        }
        let out = String::from_utf8(io.get_ref().to_vec()).unwrap();

        let ans = "\ndeclare hello\n{\n}\n\n".to_string();
        assert_eq!(out, ans);
    }

    #[test]
    fn aware_indent_01() {
        let mut b = "declare hello {input ok;}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);
        let mut io = Cursor::new(Vec::new());
        {
            let mut g = Generator::new(p, &mut io);
            g.output_node();
        }
        let out = String::from_utf8(io.get_ref().to_vec()).unwrap();

        let ans = "\ndeclare hello\n{\n\tinput ok;\n}\n\n".to_string();
        assert_eq!(out, ans);
    }

    #[test]
    fn aware_indent_02() {
        let mut b = "declare hello {input ok[2];}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);
        let mut io = Cursor::new(Vec::new());
        {
            let mut g = Generator::new(p, &mut io);
            g.output_node();
        }
        let out = String::from_utf8(io.get_ref().to_vec()).unwrap();

        let ans = "\ndeclare hello\n{\n\tinput ok[2];\n}\n\n".to_string();
        assert_eq!(out, ans);
    }

    #[test]
    fn aware_indent_03() {
        let mut b = "declare hello {input ok[OK / 2];}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);
        let mut io = Cursor::new(Vec::new());
        {
            let mut g = Generator::new(p, &mut io);
            g.output_node();
        }
        let out = String::from_utf8(io.get_ref().to_vec()).unwrap();

        let ans = "\ndeclare hello\n{\n\tinput ok[OK / 2];\n}\n\n".to_string();
        assert_eq!(out, ans);
    }

    #[test]
    fn two_arguments() {
        let mut b = "declare hello {input a; input b; func_in aa(a, b);}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);
        let mut io = Cursor::new(Vec::new());
        {
            let mut g = Generator::new(p, &mut io);
            g.output_node();
        }
        let out = String::from_utf8(io.get_ref().to_vec()).unwrap();

        let ans = "\ndeclare hello\n{\n\tinput a;\n\tinput b;\n\tfunc_in aa(a, b);\n}\n\n"
            .to_string();
        assert_eq!(out, ans);
    }

    #[test]
    fn new_by_file() {
        let mut f = BufReader::new(File::open("test_code/ugly_declare_01.nsl").unwrap());
        let mut l = Lexer::new(&mut f);

        let p = Parser::new(&mut l);
        let mut io = Cursor::new(Vec::new());
        {
            let mut g = Generator::new(p, &mut io);
            g.output_node();
        }
        let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
        let ans =
            "\ndeclare hello_google2\n{\n\tinput ok;\n\tfunc_in sugoi(ok);\n}\n\n".to_string();
        assert_eq!(out, ans);
    }

    #[test]
    fn output_declare_to_file() {
        let mut b = "declare hello {input ok; func_in hh(ok);}".as_bytes();
        let mut l = Lexer::new(&mut b);

        let p = Parser::new(&mut l);

        let file_path = format!("/tmp/{:02}.nsl", get_value_with_lock());
        let mut io = BufWriter::new(File::create(file_path).unwrap());

        {
            let mut g = Generator::new(p, &mut io);
            g.output_node();
        }
    }
}
