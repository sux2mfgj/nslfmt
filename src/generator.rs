use std::io::{Write, Error};

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

    pub fn output_node(&mut self) -> Result<usize, Error> {
        let ast = self.parser.next_ast().unwrap();
        self.writer.write(format!("{}", ast).as_bytes())
    }
}

#[cfg(test)]
mod generator_test {
    use super::*;
    use lexer::*;

    use std::fs::File;
    use std::io::{self, BufWriter, BufReader, Cursor};

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

        let ans = "declare hello\n{\n}".to_string();
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

        let ans = "declare hello\n{\n\tinput ok;\n}".to_string();
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

        let ans = "declare hello\n{\n\tinput ok[2];\n}".to_string();
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

        let ans = "declare hello\n{\n\tinput ok[OK / 2];\n}".to_string();
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

        let ans = "declare hello\n{\n\tinput a;\n\tinput b;\n\tfunc_in aa(a, b);\n}".to_string();
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
        let ans = "declare hello_google2\n{\n\tinput ok;\n\tfunc_in sugoi(ok);\n}".to_string();
        assert_eq!(out, ans);
    }
    /*

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
*/
}
