#[macro_use]
extern crate nslfmt;

extern crate backtrace;

use nslfmt::ast::*;
use nslfmt::lexer::*;
use nslfmt::parser::*;
use nslfmt::token::*;

use backtrace::Backtrace;

use std::fs::File;
use std::io::BufReader;
use std::panic;

#[cfg(test)]
mod simple_tests {
    use super::*;

    #[test]
    fn end_of_program() {
        let mut b = "".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        assert_eq!(p.next_ast(), create_node!(ASTClass::EndOfProgram));
    }
}

fn initialize() {
    panic::set_hook(Box::new(|_| {
        let bt = Backtrace::new();
        eprintln!("{:?}", bt);
    }));
}

#[cfg(test)]
mod declare {
    use super::*;

    #[test]
    fn declare_only() {
        let mut b = "declare ok {}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        assert_eq!(
            p.next_ast(),
            create_node!(ASTClass::Declare(
                create_node!(ASTClass::Identifire("ok".to_string())),
                create_node!(ASTClass::Block(vec![]))
            ))
        )
    }

    #[test]
    fn newline_in_declare_block() {
        let mut b = "declare ok {\n\n}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        assert_eq!(
            p.next_ast(),
            create_node!(ASTClass::Declare(
                create_node!(ASTClass::Identifire("ok".to_string())),
                create_node!(ASTClass::Block(vec![]))
            ))
        );
    }

    #[test]
    fn one_bit_input() {
        let mut b = "declare ok{ input a; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(create_node!(ASTClass::Input(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
        )));

        let block = create_node!(ASTClass::Block(interfaces));
        let id = create_node!(ASTClass::Identifire("ok".to_string()));
        assert_eq!(p.next_ast(), create_node!(ASTClass::Declare(id, block)));
    }

    #[test]
    fn multi_bit_input() {
        let mut b = "declare ok{ input a[2]; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(create_node!(ASTClass::Input(
            create_node!(ASTClass::Identifire("a".to_string())),
            Some(create_node!(ASTClass::Number("2".to_string())))
        )));

        let block = create_node!(ASTClass::Block(interfaces));
        let id = create_node!(ASTClass::Identifire("ok".to_string()));

        assert_eq!(p.next_ast(), create_node!(ASTClass::Declare(id, block)));
    }

    #[test]
    fn expression_in_width_block_00() {
        let mut b = "declare ok{ input a[OK]; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let left = create_node!(ASTClass::Identifire("OK".to_string()));

        let mut interfaces = Vec::new();
        interfaces.push(create_node!(ASTClass::Input(
            create_node!(ASTClass::Identifire("a".to_string())),
            Some(left),
        )));

        let id = create_node!(ASTClass::Identifire("ok".to_string()));
        let block = create_node!(ASTClass::Block(interfaces));

        assert_eq!(p.next_ast(), create_node!(ASTClass::Declare(id, block)));
    }

    /*
    // マクロは後で対応する
    #[test]
    fn macro_in_declare_00() {
        let mut b = "declare ok{ input a[2]; \nTEST_INTERFACES\n}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(create_node!(ASTClass::Input(
            create_node!(ASTClass::Identifire("a".to_string())),
            Some(create_node!(ASTClass::Number("2".to_string())))
        )));
        interfaces.push(create_node!(ASTClass::Newline));
        interfaces.push(create_node!(ASTClass::Identifire(
            "TEST_INTERFACES".to_string()
        )));
        interfaces.push(create_node!(ASTClass::Newline));

        let block = create_node!(ASTClass::Block(interfaces));
        let id = create_node!(ASTClass::Identifire("ok".to_string()));

        assert_eq!(p.next_ast(), create_node!(ASTClass::Declare(id, block)));
    }

    #[test]
    fn macro_in_declare_01() {
        let mut b =
            "declare ok{ input a[2]; \nTEST_INTERFACES\n func_in ok();}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(create_node!(ASTClass::Input(
            create_node!(ASTClass::Identifire("a".to_string())),
            Some(create_node!(ASTClass::Number("2".to_string())))
        )));
        interfaces.push(create_node!(ASTClass::Newline));
        interfaces.push(create_node!(ASTClass::Identifire(
            "TEST_INTERFACES".to_string()
        )));
        interfaces.push(create_node!(ASTClass::Newline));
        interfaces.push(create_node!(ASTClass::FuncIn(
            create_node!(ASTClass::Identifire("ok".to_string())),
            vec![],
            None
        )));

        let block = create_node!(ASTClass::Block(interfaces));
        let id = create_node!(ASTClass::Identifire("ok".to_string()));

        assert_eq!(p.next_ast(), create_node!(ASTClass::Declare(id, block)));
    }
    */

    #[test]
    fn expression_in_width_block_01() {
        let mut b = "declare ok{ input a[OK / 2]; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let left = create_node!(ASTClass::Identifire("OK".to_string()));
        let op = create_node!(ASTClass::Operator(Operator::Slash));
        let right = create_node!(ASTClass::Number("2".to_string()));
        let expr = create_node!(ASTClass::Expression(left, op, right));

        let mut interfaces = Vec::new();
        interfaces.push(create_node!(ASTClass::Input(
            create_node!(ASTClass::Identifire("a".to_string())),
            Some(expr)
        )));

        let id = create_node!(ASTClass::Identifire("ok".to_string()));
        let block = create_node!(ASTClass::Block(interfaces));

        assert_eq!(p.next_ast(), create_node!(ASTClass::Declare(id, block)));
    }

    #[test]
    fn expression_in_width_block_02() {
        let mut b = "declare ok{ input a[OK / 4 * 2]; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let left = create_node!(ASTClass::Identifire("OK".to_string()));
        let op = create_node!(ASTClass::Operator(Operator::Slash));
        let expr = create_node!(ASTClass::Expression(
            create_node!(ASTClass::Number("4".to_string())),
            create_node!(ASTClass::Operator(Operator::Asterisk)),
            create_node!(ASTClass::Number("2".to_string())),
        ));

        let top_expr = create_node!(ASTClass::Expression(left, op, expr));

        let mut interfaces = Vec::new();
        interfaces.push(create_node!(ASTClass::Input(
            create_node!(ASTClass::Identifire("a".to_string())),
            Some(top_expr),
        )));

        let id = create_node!(ASTClass::Identifire("ok".to_string()));
        let block = create_node!(ASTClass::Block(interfaces));

        assert_eq!(p.next_ast(), create_node!(ASTClass::Declare(id, block)));
    }

    #[test]
    fn output_inout() {
        let mut b = "declare ok{ output a[2]; inout b[12];}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(create_node!(ASTClass::Output(
            create_node!(ASTClass::Identifire("a".to_string())),
            Some(create_node!(ASTClass::Number("2".to_string())))
        )));

        interfaces.push(create_node!(ASTClass::InOut(
            create_node!(ASTClass::Identifire("b".to_string())),
            Some(create_node!(ASTClass::Number("12".to_string())))
        )));
        assert_eq!(
            p.next_ast(),
            create_node!(ASTClass::Declare(
                create_node!(ASTClass::Identifire("ok".to_string())),
                create_node!(ASTClass::Block(interfaces))
            ))
        );
    }

    #[test]
    fn func_in() {
        //initialize();
        let mut b = "declare ok{ input a; func_in ok(a);}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(create_node!(ASTClass::Input(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
        )));

        let args = vec![create_node!(ASTClass::Identifire("a".to_string()))];
        let func = create_node!(ASTClass::FuncIn(
            create_node!(ASTClass::Identifire("ok".to_string())),
            args,
            None,
        ));
        interfaces.push(func);

        assert_eq!(
            p.next_ast(),
            create_node!(ASTClass::Declare(
                create_node!(ASTClass::Identifire("ok".to_string())),
                create_node!(ASTClass::Block(interfaces))
            ))
        );
    }

    #[test]
    fn func_in_return() {
        let mut b = "declare ok{ input a; output c[2]; func_in ok(a): c;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(create_node!(ASTClass::Input(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
        )));
        interfaces.push(create_node!(ASTClass::Output(
            create_node!(ASTClass::Identifire("c".to_string())),
            Some(create_node!(ASTClass::Number("2".to_string())))
        )));
        let args = vec![create_node!(ASTClass::Identifire("a".to_string()))];
        let func = create_node!(ASTClass::FuncIn(
            create_node!(ASTClass::Identifire("ok".to_string())),
            args,
            Some(create_node!(ASTClass::Identifire("c".to_string())))
        ));
        interfaces.push(func);

        assert_eq!(
            p.next_ast(),
            create_node!(ASTClass::Declare(
                create_node!(ASTClass::Identifire("ok".to_string())),
                create_node!(ASTClass::Block(interfaces))
            ))
        );
    }

    #[test]
    fn func_out_return() {
        let mut b = "declare ok{ input a[3]; output c[2]; func_out ok(a): c;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(create_node!(ASTClass::Input(
            create_node!(ASTClass::Identifire("a".to_string())),
            Some(create_node!(ASTClass::Number("3".to_string())))
        )));
        interfaces.push(create_node!(ASTClass::Output(
            create_node!(ASTClass::Identifire("c".to_string())),
            Some(create_node!(ASTClass::Number("2".to_string())))
        )));
        let args = vec![create_node!(ASTClass::Identifire("a".to_string()))];
        let func = create_node!(ASTClass::FuncOut(
            create_node!(ASTClass::Identifire("ok".to_string())),
            args,
            Some(create_node!(ASTClass::Identifire("c".to_string())))
        ));
        interfaces.push(func);

        assert_eq!(
            p.next_ast(),
            create_node!(ASTClass::Declare(
                create_node!(ASTClass::Identifire("ok".to_string())),
                create_node!(ASTClass::Block(interfaces))
            ))
        );
    }

    #[test]
    fn declare_03() {
        let mut b = BufReader::new(File::open("nsl_samples/declare_03.nsl").unwrap());
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(create_node!(ASTClass::Input(
            create_node!(ASTClass::Identifire("ok".to_string())),
            None,
        )));
        interfaces.push(create_node!(ASTClass::Input(
            create_node!(ASTClass::Identifire("ggrks".to_string())),
            None,
        )));
        interfaces.push(create_node!(ASTClass::Output(
            create_node!(ASTClass::Identifire("jk".to_string())),
            None,
        )));

        let args1 = vec![create_node!(ASTClass::Identifire("ok".to_string()))];
        let func1 = create_node!(ASTClass::FuncIn(
            create_node!(ASTClass::Identifire("sugoi".to_string())),
            args1,
            None,
        ));

        let args2 = vec![create_node!(ASTClass::Identifire("jk".to_string()))];
        let func2 = create_node!(ASTClass::FuncOut(
            create_node!(ASTClass::Identifire("majika".to_string())),
            args2,
            Some(create_node!(ASTClass::Identifire("ggrks".to_string())))
        ));

        interfaces.push(func1);
        interfaces.push(func2);

        assert_eq!(
            p.next_ast(),
            create_node!(ASTClass::Declare(
                create_node!(ASTClass::Identifire("hel".to_string())),
                create_node!(ASTClass::Block(interfaces))
            ))
        );
    }

}

#[cfg(test)]
mod macros {
    use super::*;

    #[test]
    fn include_macro() {
        let mut b = "#include \"hello.h\"\ndeclare ok {}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let path = create_node!(ASTClass::String("hello.h".to_string()));
        let _id = create_node!(ASTClass::Declare(
            create_node!(ASTClass::Identifire("ok".to_string())),
            create_node!(ASTClass::Block(Vec::new()))
        ));
        let include = create_node!(ASTClass::MacroInclude(path));
        assert_eq!(p.next_ast(), include);
    }

    #[test]
    fn undef_macro() {
        let mut b = "#undef hello".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let undef = create_node!(ASTClass::MacroUndef(create_node!(
            ASTClass::Identifire("hello".to_string())
        )));
        assert_eq!(p.next_ast(), undef);
    }

    #[test]
    fn ifdef_macro() {
        let mut b = "#ifdef hello".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let ifdef = create_node!(ASTClass::MacroIfdef(create_node!(
            ASTClass::Identifire("hello".to_string())
        )));
        assert_eq!(p.next_ast(), ifdef);
    }

    #[test]
    fn ifndef_macro() {
        let mut b = "#ifndef hello".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let ifndef = create_node!(ASTClass::MacroIfndef(create_node!(
            ASTClass::Identifire("hello".to_string())
        )));
        assert_eq!(p.next_ast(), ifndef);
    }

    #[test]
    fn endif_macro() {
        let mut b = "#ifndef hello\n#endif".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let ifndef = create_node!(ASTClass::MacroIfndef(create_node!(
            ASTClass::Identifire("hello".to_string())
        )));
        let endif = create_node!(ASTClass::MacroEndif);
        assert_eq!(p.next_ast(), ifndef);
        assert_eq!(p.next_ast(), endif);
    }

    #[test]
    fn if_else_end() {
        let mut b = "#ifndef hello\n#else\n#endif".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let ifndef = create_node!(ASTClass::MacroIfndef(create_node!(
            ASTClass::Identifire("hello".to_string())
        )));
        let endif = create_node!(ASTClass::MacroEndif);
        let melse = create_node!(ASTClass::MacroElse);
        assert_eq!(p.next_ast(), ifndef);
        assert_eq!(p.next_ast(), melse);
        assert_eq!(p.next_ast(), endif);
    }

    #[test]
    fn define_macro_nl() {
        let mut b = "#define HELLO input ok;\n".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let def_macro = create_node!(ASTClass::MacroDefine(
            create_node!(ASTClass::Identifire("HELLO".to_string())),
            Some("input ok;".to_string())
        ));
        assert_eq!(p.next_ast(), def_macro);
    }

    #[test]
    fn define_macro_eof() {
        let mut b = "#define HELLO input ok;".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let def_macro = create_node!(ASTClass::MacroDefine(
            create_node!(ASTClass::Identifire("HELLO".to_string())),
            Some("input ok;".to_string())
        ));
        assert_eq!(p.next_ast(), def_macro);
    }

    #[test]
    fn define_macro2() {
        // axi4 master interface
        let mut b = "#define AXI4_LITE_MASTER_INTERFACE output awvalid; input awready; output awaddr[AXI_ADDR_WIDTH]; output awprot[3]; output wvalid; input wready; output wdata[AXI_DATA_WIDTH]; output wstrb[AXI_DATA_WIDTH / 8]; input bvalid; output bready; input bresp[2]; output arvalid; input arready; output araddr[AXI_ADDR_WIDTH]; output arprot[3]; input rvalid; output rready; input rdata[AXI_DATA_WIDTH]; input rresp[2];".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let def_macro = create_node!(ASTClass::MacroDefine(
                create_node!(ASTClass::Identifire("AXI4_LITE_MASTER_INTERFACE".to_string())),
                Some("output awvalid; input awready; output awaddr[ AXI_ADDR_WIDTH ]; output awprot[ 3 ]; output wvalid; input wready; output wdata[ AXI_DATA_WIDTH ]; output wstrb[ AXI_DATA_WIDTH / 8 ]; input bvalid; output bready; input bresp[ 2 ]; output arvalid; input arready; output araddr[ AXI_ADDR_WIDTH ]; output arprot[ 3 ]; input rvalid; output rready; input rdata[ AXI_DATA_WIDTH ]; input rresp[ 2 ];".to_string())));

        assert_eq!(p.next_ast(), def_macro);
    }

    #[test]
    fn define_macro3() {
        let mut b = "#define HELLO_ONLY".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let def_macro = create_node!(ASTClass::MacroDefine(
            create_node!(ASTClass::Identifire("HELLO_ONLY".to_string())),
            None
        ));
        assert_eq!(p.next_ast(), def_macro);
    }
}

#[cfg(test)]
mod comment {
    use super::*;

    #[test]
    fn multi_line_comment() {
        let mut b = "/*\ndata lines\n*/".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let multi_line = create_node!(ASTClass::CStyleComment(vec![
            "".to_string(),
            "data lines".to_string(),
            "".to_string(),
        ]));

        assert_eq!(p.next_ast(), multi_line);
    }
}

#[cfg(test)]
mod module {
    use super::*;

    #[test]
    fn module_00() {
        let mut b = "module test {}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let components = vec![];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn wire_01() {
        let mut b = "module test {wire a;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        // wire data, a[12];
        //Wire<Vec<(String, String)>
        let wire_def = create_node!(ASTClass::Wire(vec![(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
        )]));
        let components = vec![wire_def];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn wire_02() {
        let mut b = "module test {wire a, b;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        // wire data, a[12];
        //Wire<Vec<(String, String)>
        let wire_def = create_node!(ASTClass::Wire(vec![
            (create_node!(ASTClass::Identifire("a".to_string())), None),
            (create_node!(ASTClass::Identifire("b".to_string())), None),
        ]));
        let components = vec![wire_def];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn wire_03() {
        let mut b = "module test {wire a[12], b;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        // wire data, a[12];
        //Wire<Vec<(String, String)>
        let wire_def = create_node!(ASTClass::Wire(vec![
            (
                create_node!(ASTClass::Identifire("a".to_string())),
                Some(create_node!(ASTClass::Number("12".to_string()))),
            ),
            (create_node!(ASTClass::Identifire("b".to_string())), None),
        ]));
        let components = vec![wire_def];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn wire_04() {
        let mut b = "module test {wire a[12], b[23];}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        // wire data, a[12];
        //Wire<Vec<(String, String)>
        let wire_def = create_node!(ASTClass::Wire(vec![
            (
                create_node!(ASTClass::Identifire("a".to_string())),
                Some(create_node!(ASTClass::Number("12".to_string()))),
            ),
            (
                create_node!(ASTClass::Identifire("b".to_string())),
                Some(create_node!(ASTClass::Number("23".to_string()))),
            ),
        ]));
        let components = vec![wire_def];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn wire_05() {
        let mut b = "module test {wire a[12], b[23], c;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        // wire data, a[12];
        //Wire<Vec<(String, String)>
        let wire_def = create_node!(ASTClass::Wire(vec![
            (
                create_node!(ASTClass::Identifire("a".to_string())),
                Some(create_node!(ASTClass::Number("12".to_string()))),
            ),
            (
                create_node!(ASTClass::Identifire("b".to_string())),
                Some(create_node!(ASTClass::Number("23".to_string()))),
            ),
            (create_node!(ASTClass::Identifire("c".to_string())), None),
        ]));
        let components = vec![wire_def];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn reg_00() {
        let mut b = "module test { reg a; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let reg_def = create_node!(ASTClass::Reg(vec![(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
            None,
        )]));
        let components = vec![reg_def];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn reg_01() {
        let mut b = "module test { reg a, b; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let reg_def = create_node!(ASTClass::Reg(vec![
            (
                create_node!(ASTClass::Identifire("a".to_string())),
                None,
                None,
            ),
            (
                create_node!(ASTClass::Identifire("b".to_string())),
                None,
                None,
            ),
        ]));
        let components = vec![reg_def];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn reg_02() {
        let mut b = "module test { reg a[12], b; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let reg_def = create_node!(ASTClass::Reg(vec![
            (
                create_node!(ASTClass::Identifire("a".to_string())),
                Some(create_node!(ASTClass::Number("12".to_string()))),
                None,
            ),
            (
                create_node!(ASTClass::Identifire("b".to_string())),
                None,
                None,
            ),
        ]));
        let components = vec![reg_def];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn reg_03() {
        let mut b = "module test { reg a[4] = 4'b1001, b; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let reg_def = create_node!(ASTClass::Reg(vec![
            (
                create_node!(ASTClass::Identifire("a".to_string())),
                Some(create_node!(ASTClass::Number("4".to_string()))),
                Some(create_node!(ASTClass::Number("4'b1001".to_string()))),
            ),
            (
                create_node!(ASTClass::Identifire("b".to_string())),
                None,
                None,
            ),
        ]));
        let components = vec![reg_def];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn reg_04() {
        let mut b = "module test { reg a[4] = 4'b1001, b[12]; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let reg_def = create_node!(ASTClass::Reg(vec![
            (
                create_node!(ASTClass::Identifire("a".to_string())),
                Some(create_node!(ASTClass::Number("4".to_string()))),
                Some(create_node!(ASTClass::Number("4'b1001".to_string()))),
            ),
            (
                create_node!(ASTClass::Identifire("b".to_string())),
                Some(create_node!(ASTClass::Number("12".to_string()))),
                None,
            ),
        ]));
        let components = vec![reg_def];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn reg_05() {
        let mut b = "module test { reg a = 1'b1, b[12]; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let reg_def = create_node!(ASTClass::Reg(vec![
            (
                create_node!(ASTClass::Identifire("a".to_string())),
                None,
                Some(create_node!(ASTClass::Number("1'b1".to_string()))),
            ),
            (
                create_node!(ASTClass::Identifire("b".to_string())),
                Some(create_node!(ASTClass::Number("12".to_string()))),
                None,
            ),
        ]));
        let components = vec![reg_def];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn reg_06() {
        let mut b = "module test { reg a = 1'b1;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let reg_def = create_node!(ASTClass::Reg(vec![(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
            Some(create_node!(ASTClass::Number("1'b1".to_string()))),
        )]));
        let components = vec![reg_def];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn func_self_00() {
        let mut b = "module test { func_self aa;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let func_self = create_node!(ASTClass::FuncSelf(
            create_node!(ASTClass::Identifire("aa".to_string())),
            vec![],
            None,
        ));

        let components = vec![func_self];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn func_self_01() {
        let mut b = "module test { wire a, b; func_self aa(a, b);}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let wire_def = create_node!(ASTClass::Wire(vec![
            (create_node!(ASTClass::Identifire("a".to_string())), None),
            (create_node!(ASTClass::Identifire("b".to_string())), None),
        ]));

        let func_self = create_node!(ASTClass::FuncSelf(
            create_node!(ASTClass::Identifire("aa".to_string())),
            vec![
                create_node!(ASTClass::Identifire("a".to_string())),
                create_node!(ASTClass::Identifire("b".to_string())),
            ],
            None,
        ));

        let components = vec![wire_def, func_self];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn func_self_02() {
        let mut b = "module test { wire a; func_self aa: a;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let wire_def = create_node!(ASTClass::Wire(vec![(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
        )]));

        let func_self = create_node!(ASTClass::FuncSelf(
            create_node!(ASTClass::Identifire("aa".to_string())),
            vec![],
            Some(create_node!(ASTClass::Identifire("a".to_string()))),
        ));

        let components = vec![wire_def, func_self];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn func_self_03() {
        let mut b = "module test { wire a, b; func_self aa(a): b;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let wire_def = create_node!(ASTClass::Wire(vec![
            (create_node!(ASTClass::Identifire("a".to_string())), None),
            (create_node!(ASTClass::Identifire("b".to_string())), None),
        ]));

        let func_self = create_node!(ASTClass::FuncSelf(
            create_node!(ASTClass::Identifire("aa".to_string())),
            vec![create_node!(ASTClass::Identifire("a".to_string()))],
            Some(create_node!(ASTClass::Identifire("b".to_string()))),
        ));

        let components = vec![wire_def, func_self];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn proc_00() {
        let mut b = "module test { proc_name proc_a(); }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let components = vec![create_node!(ASTClass::ProcName(
            create_node!(ASTClass::Identifire("proc_a".to_string())),
            vec![]
        ))];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn proc_01() {
        let mut b = "module test { reg r1; proc_name proc_a(r1); }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let reg_def = create_node!(ASTClass::Reg(vec![(
            create_node!(ASTClass::Identifire("r1".to_string())),
            None,
            None,
        )]));

        let components = vec![
            reg_def,
            create_node!(ASTClass::ProcName(
                create_node!(ASTClass::Identifire("proc_a".to_string())),
                vec![create_node!(ASTClass::Identifire("r1".to_string()))]
            )),
        ];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn state_name_00() {
        let mut b = "module test { state_name state1;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let components = vec![create_node!(ASTClass::StateName(vec![create_node!(
            ASTClass::Identifire("state1".to_string())
        )]))];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn state_name_01() {
        let mut b = "module test { state_name state1, state2;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let components = vec![create_node!(ASTClass::StateName(vec![
            create_node!(ASTClass::Identifire("state1".to_string())),
            create_node!(ASTClass::Identifire("state2".to_string())),
        ]))];

        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn state_procedure_00() {
        let mut b =
            "module test { state_name state1, state2; state state1 {}}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let components = vec![
            create_node!(ASTClass::StateName(vec![
                create_node!(ASTClass::Identifire("state1".to_string())),
                create_node!(ASTClass::Identifire("state2".to_string())),
            ])),
            create_node!(ASTClass::State(
                create_node!(ASTClass::Identifire("state1".to_string())),
                create_node!(ASTClass::Block(vec![]))
            )),
        ];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn state_procedure_01() {
        let mut b = "module test { state state1 {a = 1'b1;}}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let state_block = vec![create_node!(ASTClass::Assign(
            create_node!(ASTClass::Identifire("a".to_string())),
            create_node!(ASTClass::Number("1'b1".to_string())),
        ))];

        let components = vec![create_node!(ASTClass::State(
            create_node!(ASTClass::Identifire("state1".to_string())),
            create_node!(ASTClass::Block(state_block))
        ))];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn state_procedure_02() {
        let mut b = "module test { state state1 {error(a);}}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let state_block = vec![create_node!(ASTClass::FuncCall(
            create_node!(ASTClass::Identifire("error".to_string())),
            vec![create_node!(ASTClass::Identifire("a".to_string()))],
        ))];

        let components = vec![create_node!(ASTClass::State(
            create_node!(ASTClass::Identifire("state1".to_string())),
            create_node!(ASTClass::Block(state_block))
        ))];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn mem_00() {
        let mut b = "module test {mem aa[12];}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mem = create_node!(ASTClass::Mem(vec![(
            create_node!(ASTClass::Identifire("aa".to_string())),
            create_node!(ASTClass::Number("12".to_string())),
            None,
            None,
        )]));
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(vec![mem]))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn mem_01() {
        let mut b = "module test {mem aa[1] = {1'b1};}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mem = create_node!(ASTClass::Mem(vec![(
            create_node!(ASTClass::Identifire("aa".to_string())),
            create_node!(ASTClass::Number("1".to_string())),
            None,
            Some(vec![create_node!(ASTClass::Number("1'b1".to_string()))]),
        )]));
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(vec![mem]))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn mem_02() {
        let mut b = "module test {mem aa[4][2] = {4'b1010, 4'b0101};}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mem = create_node!(ASTClass::Mem(vec![(
            create_node!(ASTClass::Identifire("aa".to_string())),
            create_node!(ASTClass::Number("4".to_string())),
            Some(create_node!(ASTClass::Number("2".to_string()))),
            Some(vec![
                create_node!(ASTClass::Number("4'b1010".to_string())),
                create_node!(ASTClass::Number("4'b0101".to_string())),
            ]),
        )]));
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(vec![mem]))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn mem_03() {
        let mut b = "module test {mem aa[4][2];}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);
        let mem = create_node!(ASTClass::Mem(vec![(
            create_node!(ASTClass::Identifire("aa".to_string())),
            create_node!(ASTClass::Number("4".to_string())),
            Some(create_node!(ASTClass::Number("2".to_string()))),
            None,
        )]));
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(vec![mem]))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn mem_04() {
        let mut b =
            "mem a[12] = {1'b1, 1'b0, 1'b0, 1'b1}, b[3][4] = {4'b1110};".as_bytes();
        let mut l = Lexer::new(&mut b);
        let p = Parser::new(&mut l);
        let mem = create_node!(ASTClass::Mem(vec![
            (
                create_node!(ASTClass::Identifire("a".to_string())),
                create_node!(ASTClass::Number("12".to_string())),
                None,
                Some(vec![
                    create_node!(ASTClass::Number("1'b1".to_string())),
                    create_node!(ASTClass::Number("1'b0".to_string())),
                    create_node!(ASTClass::Number("1'b0".to_string())),
                    create_node!(ASTClass::Number("1'b1".to_string())),
                ]),
            ),
            (
                create_node!(ASTClass::Identifire("b".to_string())),
                create_node!(ASTClass::Number("3".to_string())),
                Some(create_node!(ASTClass::Number("4".to_string()))),
                Some(vec![create_node!(ASTClass::Number("4'b1110".to_string()))]),
            ),
        ]));
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(vec![mem]))
        ));
    }

    #[test]
    fn wire_assign_00() {
        let mut b = "module test { wire a; a = 1'b1;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let wire = create_node!(ASTClass::Wire(vec![(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
        )]));
        let assign = create_node!(ASTClass::Assign(
            create_node!(ASTClass::Identifire("a".to_string())),
            create_node!(ASTClass::Number("1'b1".to_string())),
        ));
        let components = vec![wire, assign];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn wire_assign_01() {
        let mut b = "module test { wire a; a = a + 1'b1;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let wire = create_node!(ASTClass::Wire(vec![(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
        )]));

        let expr = create_node!(ASTClass::Expression(
            create_node!(ASTClass::Identifire("a".to_string())),
            create_node!(ASTClass::Operator(Operator::Plus)),
            create_node!(ASTClass::Number("1'b1".to_string())),
        ));
        let assign = create_node!(ASTClass::Assign(
            create_node!(ASTClass::Identifire("a".to_string())),
            expr,
        ));
        let components = vec![wire, assign];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn wire_assign_02() {
        let mut b = "module test { wire a; a = a + a;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let wire = create_node!(ASTClass::Wire(vec![(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
        )]));

        let expr = create_node!(ASTClass::Expression(
            create_node!(ASTClass::Identifire("a".to_string())),
            create_node!(ASTClass::Operator(Operator::Plus)),
            create_node!(ASTClass::Identifire("a".to_string())),
        ));
        let assign = create_node!(ASTClass::Assign(
            create_node!(ASTClass::Identifire("a".to_string())),
            expr,
        ));
        let components = vec![wire, assign];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn wire_assign_03() {
        let mut b = "module test { wire a; a = a + 1'b1;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let wire = create_node!(ASTClass::Wire(vec![(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
        )]));

        let expr = create_node!(ASTClass::Expression(
            create_node!(ASTClass::Identifire("a".to_string())),
            create_node!(ASTClass::Operator(Operator::Plus)),
            create_node!(ASTClass::Number("1'b1".to_string())),
        ));
        let assign = create_node!(ASTClass::Assign(
            create_node!(ASTClass::Identifire("a".to_string())),
            expr,
        ));
        let components = vec![wire, assign];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn reg_assign_00() {
        let mut b = "module test { reg a; a := a + 1'b1;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let wire = create_node!(ASTClass::Reg(vec![(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
            None,
        )]));

        let expr = create_node!(ASTClass::Expression(
            create_node!(ASTClass::Identifire("a".to_string())),
            create_node!(ASTClass::Operator(Operator::Plus)),
            create_node!(ASTClass::Number("1'b1".to_string())),
        ));
        let assign = create_node!(ASTClass::RegAssign(
            create_node!(ASTClass::Identifire("a".to_string())),
            expr,
        ));
        let components = vec![wire, assign];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn func_in_module_00() {
        let mut b = "module test { func ok {} }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let components = vec![create_node!(ASTClass::Func(
            create_node!(ASTClass::Identifire("ok".to_string())),
            None,
            create_node!(ASTClass::Block(vec![]))
        ))];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn func_in_module_01() {
        let mut b = "module test { func ok.enable {} }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let components = vec![create_node!(ASTClass::Func(
            create_node!(ASTClass::Identifire("ok".to_string())),
            Some(create_node!(ASTClass::Identifire("enable".to_string()))),
            create_node!(ASTClass::Block(vec![]))
        ))];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn return_00() {
        let mut b = "module test { func ok { return mtvec;} }".as_bytes();

        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let func_components = vec![create_node!(ASTClass::Return(create_node!(
            ASTClass::Identifire("mtvec".to_string())
        )))];
        let components = vec![create_node!(ASTClass::Func(
            create_node!(ASTClass::Identifire("ok".to_string())),
            None,
            create_node!(ASTClass::Block(func_components))
        ))];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn any_00() {
        let mut b = "module test { any {} }".as_bytes();

        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let components = vec![create_node!(ASTClass::Any(vec![]))];

        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
        assert_eq!(p.next_ast(), create_node!(ASTClass::EndOfProgram));
    }

    #[test]
    fn any_01() {
        let mut b = "module test { reg a = 0; any {a : { a := 1; }} }".as_bytes();

        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let any_comp = vec![(
            create_node!(ASTClass::Identifire("a".to_string())),
            create_node!(ASTClass::Block(vec![create_node!(ASTClass::RegAssign(
                create_node!(ASTClass::Identifire("a".to_string())),
                create_node!(ASTClass::Number("1".to_string()))
            ))])),
        )];
        let reg = create_node!(ASTClass::Reg(vec![(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
            Some(create_node!(ASTClass::Number("0".to_string()))),
        )]));
        let components = vec![reg, create_node!(ASTClass::Any(any_comp))];

        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
        assert_eq!(p.next_ast(), create_node!(ASTClass::EndOfProgram));
    }

    #[test]
    fn any_02() {
        let mut b =
            "module test { reg a = 0; any {a : { a := 1; } a == 1'b1: { a := 0;}} }"
                .as_bytes();

        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let left = create_node!(ASTClass::Identifire("a".to_string()));
        let op = create_node!(ASTClass::Operator(Operator::Equal));
        let right = create_node!(ASTClass::Number("1'b1".to_string()));
        let expr = create_node!(ASTClass::Expression(left, op, right));

        let any_comp = vec![
            (
                create_node!(ASTClass::Identifire("a".to_string())),
                create_node!(ASTClass::Block(vec![create_node!(ASTClass::RegAssign(
                    create_node!(ASTClass::Identifire("a".to_string())),
                    create_node!(ASTClass::Number("1".to_string()))
                ))])),
            ),
            (
                expr,
                create_node!(ASTClass::Block(vec![create_node!(ASTClass::RegAssign(
                    create_node!(ASTClass::Identifire("a".to_string())),
                    create_node!(ASTClass::Number("0".to_string()))
                ))])),
            ),
        ];
        let reg = create_node!(ASTClass::Reg(vec![(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
            Some(create_node!(ASTClass::Number("0".to_string()))),
        )]));
        let components = vec![reg, create_node!(ASTClass::Any(any_comp))];

        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn any_else_00() {
        let mut b = "module test { reg a = 0; any {a : {} else: {}} }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let any_comp = vec![
            (
                create_node!(ASTClass::Identifire("a".to_string())),
                create_node!(ASTClass::Block(vec![])),
            ),
            (
                create_node!(ASTClass::Else),
                create_node!(ASTClass::Block(vec![])),
            ),
        ];
        let reg = create_node!(ASTClass::Reg(vec![(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
            Some(create_node!(ASTClass::Number("0".to_string()))),
        )]));
        let components = vec![reg, create_node!(ASTClass::Any(any_comp))];

        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn function_call_00() {
        let mut b = "module test {error();}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let components = vec![create_node!(ASTClass::FuncCall(
            create_node!(ASTClass::Identifire("error".to_string())),
            vec![],
        ))];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn function_call_01() {
        let mut b = "module test {error(a);}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let components = vec![create_node!(ASTClass::FuncCall(
            create_node!(ASTClass::Identifire("error".to_string())),
            vec![create_node!(ASTClass::Identifire("a".to_string()))],
        ))];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
        assert_eq!(p.next_ast(), create_node!(ASTClass::EndOfProgram));
    }

    #[test]
    fn func_in_00() {
        let mut b = "module hello {wire a;func ok {a = 1'b0;}}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let wire_def = create_node!(ASTClass::Wire(vec![(
            create_node!(ASTClass::Identifire("a".to_string())),
            None,
        )]));
        let func_block =
            create_node!(ASTClass::Block(vec![create_node!(ASTClass::Assign(
                create_node!(ASTClass::Identifire("a".to_string())),
                create_node!(ASTClass::Number("1'b0".to_string()))
            ))]));
        let func_def = create_node!(ASTClass::Func(
            create_node!(ASTClass::Identifire("ok".to_string())),
            None,
            func_block,
        ));

        let components = vec![wire_def, func_def];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("hello".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn reg_func_00() {
        let mut b = "module test {mhartid := update(funct, mhartid, source);}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let func_call = create_node!(ASTClass::FuncCall(
            create_node!(ASTClass::Identifire("update".to_string())),
            vec![
                create_node!(ASTClass::Identifire("funct".to_string())),
                create_node!(ASTClass::Identifire("mhartid".to_string())),
                create_node!(ASTClass::Identifire("source".to_string())),
            ]
        ));
        let assign = create_node!(ASTClass::RegAssign(
            create_node!(ASTClass::Identifire("mhartid".to_string())),
            func_call,
        ));

        let components = vec![assign];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn func_call_00() {
        let mut b = "module test {update(12'h123);}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let func_call = create_node!(ASTClass::FuncCall(
            create_node!(ASTClass::Identifire("update".to_string())),
            vec![create_node!(ASTClass::Number("12'h123".to_string()))]
        ));
        let components = vec![func_call];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn reg_func_01() {
        let mut b =
            "module test {mhartid := update(funct, mhartid, source) + 4'b1;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let func_call = create_node!(ASTClass::FuncCall(
            create_node!(ASTClass::Identifire("update".to_string())),
            vec![
                create_node!(ASTClass::Identifire("funct".to_string())),
                create_node!(ASTClass::Identifire("mhartid".to_string())),
                create_node!(ASTClass::Identifire("source".to_string())),
            ]
        ));
        let expressiton = create_node!(ASTClass::Expression(
            func_call,
            create_node!(ASTClass::Operator(Operator::Plus)),
            create_node!(ASTClass::Number("4'b1".to_string()))
        ));
        let assign = create_node!(ASTClass::RegAssign(
            create_node!(ASTClass::Identifire("mhartid".to_string())),
            expressiton,
        ));

        let components = vec![assign];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn if_00() {
        let mut b = "module test { if(a) {}}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let if_node = create_node!(ASTClass::If(
            create_node!(ASTClass::Identifire("a".to_string())),
            create_node!(ASTClass::Block(vec![])),
            None,
        ));
        let components = vec![if_node];

        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components)),
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn if_01() {
        let mut b = "module test { if(a) {b = 12'h123;}}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let assign = create_node!(ASTClass::Assign(
            create_node!(ASTClass::Identifire("b".to_string())),
            create_node!(ASTClass::Number("12'h123".to_string()))
        ));
        let if_node = create_node!(ASTClass::If(
            create_node!(ASTClass::Identifire("a".to_string())),
            create_node!(ASTClass::Block(vec![assign])),
            None,
        ));
        let components = vec![if_node];

        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components)),
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn if_02() {
        let mut b = "module test { if(a) b = 12'h123;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let assign = create_node!(ASTClass::Assign(
            create_node!(ASTClass::Identifire("b".to_string())),
            create_node!(ASTClass::Number("12'h123".to_string()))
        ));

        let if_node = create_node!(ASTClass::If(
            create_node!(ASTClass::Identifire("a".to_string())),
            assign,
            None
        ));
        let components = vec![if_node];

        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components)),
        ));
        assert_eq!(p.next_ast(), module);
    }

    #[test]
    fn if_03() {
        let mut b = "module test { if(a) {b = 12'h123;} else {}}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let assign = create_node!(ASTClass::Assign(
            create_node!(ASTClass::Identifire("b".to_string())),
            create_node!(ASTClass::Number("12'h123".to_string()))
        ));
        let if_node = create_node!(ASTClass::If(
            create_node!(ASTClass::Identifire("a".to_string())),
            assign,
            None
        ));
        let components = vec![if_node];
        let module = create_node!(ASTClass::Module(
            create_node!(ASTClass::Identifire("test".to_string())),
            create_node!(ASTClass::Block(components))
        ));

        assert_eq!(p.next_ast(), module);
    }
}
