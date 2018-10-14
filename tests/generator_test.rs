extern crate nslfmt;

use nslfmt::generator::*;
use nslfmt::lexer::*;
use nslfmt::parser::*;

use std::fs::File;
use std::io::{BufReader, BufWriter, Cursor};
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
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();

    let ans = "declare hello\n{\n}\n".to_string();
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
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();

    let ans = "declare hello\n{\n    input ok;\n}\n".to_string();
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
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();

    let ans = "declare hello\n{\n    input ok[2];\n}\n".to_string();
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
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();

    let ans = "declare hello\n{\n    input ok[OK / 2];\n}\n".to_string();
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
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();

    let ans = "declare hello\n{\n    input a;\n    input b;\n    func_in aa(a, b);\n}\n"
        .to_string();
    assert_eq!(out, ans);
}

#[test]
fn new_by_file() {
    let mut f = BufReader::new(File::open("nsl_samples/ugly_declare_01.nsl").unwrap());
    let mut l = Lexer::new(&mut f);

    let p = Parser::new(&mut l);
    let mut io = Cursor::new(Vec::new());
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
    let ans = "declare hello_google2\n{\n    input ok;\n    func_in sugoi(ok);\n}\n"
        .to_string();
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
        g.output_node().unwrap();
    }
}

#[test]
fn parences() {
    let mut b = "#define HELLO ( 12 )".as_bytes();
    let mut l = Lexer::new(&mut b);

    let p = Parser::new(&mut l);
    let mut io = Cursor::new(Vec::new());
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
    let ans = "#define HELLO ( 12 )\n".to_string();
    assert_eq!(out, ans);
}

#[test]
fn define_path() {
    let mut b = "#define MEMORY_HEX \"../hexs/rv32ui-p-xori.hex\"".as_bytes();
    let mut l = Lexer::new(&mut b);

    let p = Parser::new(&mut l);
    let mut io = Cursor::new(Vec::new());
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
    let ans = "#define MEMORY_HEX \"../hexs/rv32ui-p-xori.hex\"\n".to_string();
    assert_eq!(out, ans);
}

#[test]
fn comment_00() {
    let mut b = "/*\ndata lines\n*/\n".as_bytes();
    let mut l = Lexer::new(&mut b);

    let p = Parser::new(&mut l);
    let mut io = Cursor::new(Vec::new());
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
    let ans = "/*\ndata lines\n*/\n".to_string();
    assert_eq!(out, ans);
}

#[test]
fn reg_00() {
    let mut b = "module hello {reg ok;}".as_bytes();
    let mut l = Lexer::new(&mut b);
    let p = Parser::new(&mut l);
    let mut io = Cursor::new(Vec::new());
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
    let ans = "module hello\n{\n    reg ok;\n}\n".to_string();
    assert_eq!(out, ans);
}

#[test]
fn wire_00() {
    let mut b = "module hello {\n  wire ok;\n}".as_bytes();
    let mut l = Lexer::new(&mut b);
    let p = Parser::new(&mut l);
    let mut io = Cursor::new(Vec::new());
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
    let ans = "module hello\n{\n    wire ok;\n}\n".to_string();
    assert_eq!(out, ans);
}

#[test]
fn wire_01() {
    let mut b = "module hello {\n  wire ok\n, jk[\n23];\n}".as_bytes();
    let mut l = Lexer::new(&mut b);
    let p = Parser::new(&mut l);
    let mut io = Cursor::new(Vec::new());
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
    let ans = "module hello\n{\n    wire ok, jk[23];\n}\n".to_string();
    assert_eq!(out, ans);
}

#[test]
fn func_self_00() {
    let mut b = "module hello {wire update_funct[2], update_result[32]; func_self update(update_funct): update_result;}".as_bytes();
    let mut l = Lexer::new(&mut b);
    let p = Parser::new(&mut l);
    let mut io = Cursor::new(Vec::new());
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
    let ans = "module hello\n{\n    wire update_funct[2], update_result[32];\n    func_self update(update_funct) : update_result;\n}\n".to_string();
    assert_eq!(out, ans);
}

#[test]
fn func_call_00() {
    let mut b = "module hello { error(12'bf3f); }".as_bytes();
    let mut l = Lexer::new(&mut b);
    let p = Parser::new(&mut l);
    let mut io = Cursor::new(Vec::new());
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
    let ans = "module hello\n{\n    error(12'bf3f);\n}\n".to_string();
    assert_eq!(out, ans);
}

#[test]
fn func_00() {
    let mut b = "module hello {func ok {error();}}".as_bytes();
    let mut l = Lexer::new(&mut b);
    let p = Parser::new(&mut l);
    let mut io = Cursor::new(Vec::new());
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
    let ans = "module hello\n{\n    func ok\n{\n    error();\n}\n}\n".to_string();
    assert_eq!(out, ans);
}

#[test]
fn return_00() {
    let mut b = "module hello {func ok {return a;}}".as_bytes();
    let mut l = Lexer::new(&mut b);
    let p = Parser::new(&mut l);
    let mut io = Cursor::new(Vec::new());
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
    let ans = "module hello\n{\n    func ok\n{\n    return a;\n}\n}\n".to_string();
    assert_eq!(out, ans);
}

#[test]
fn assign_wire_00() {
    let mut b = "module hello {wire a;func ok {a = 1'b0;}}".as_bytes();
    let mut l = Lexer::new(&mut b);
    let p = Parser::new(&mut l);
    let mut io = Cursor::new(Vec::new());
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
    let ans = "module hello\n{\n    wire a;\n    func ok\n{\n    a = 1'b0;\n}\n}\n".to_string();
    assert_eq!(out, ans);
}

#[test]
fn any_00() {
    let mut b = "module test { reg a = 0; any {a : \n{ a := 1; }} }".as_bytes();
    let mut l = Lexer::new(&mut b);
    let p = Parser::new(&mut l);
    let mut io = Cursor::new(Vec::new());
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
    let ans = "module test\n{\n    reg a = 0;\n    any\n{\na:\n{\n    a := 1;\n}\n}\n}\n".to_string();
    assert_eq!(out, ans);
}

#[test]
fn comment_01() {
    let mut b = "// hello".as_bytes();
    let mut l = Lexer::new(&mut b);
    let p = Parser::new(&mut l);
    let mut io = Cursor::new(Vec::new());
    {
        let mut g = Generator::new(p, &mut io);
        g.output_node().unwrap();
    }
    let out = String::from_utf8(io.get_ref().to_vec()).unwrap();
    let ans = "// hello\n".to_string();
    assert_eq!(out, ans);
}

