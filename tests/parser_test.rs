#[macro_use]
extern crate nslfmt;

use nslfmt::lexer::*;
use nslfmt::token::*;
use nslfmt::parser::*;
use nslfmt::ast::*;

use std::fs::File;
use std::io::BufReader;

#[test]
fn end_of_program() {
    let mut b = "".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    assert_eq!(p.next_ast().err(), None);
}

#[test]
fn declare_only () {
    let mut b = "declare ok {}".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    assert_eq!(
        p.next_ast().unwrap(),
        create_node!(ASTClass::Declare(
                create_node!(ASTClass::Identifire("ok".to_string())),
                create_node!(ASTClass::Block(vec![], 1))))
        )
}

#[test]
fn one_bit_input() {
    let mut b = "declare ok{ input a; }".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let mut interfaces = Vec::new();
    interfaces.push(create_node!(ASTClass::Input(
        create_node!(ASTClass::Identifire("a".to_string())),
        create_node!(ASTClass::Number("1".to_string()))
    )));

    let block = create_node!(ASTClass::Block(interfaces, 1));
    let id = create_node!(ASTClass::Identifire("ok".to_string()));
    assert_eq!(
        p.next_ast().unwrap(),
        create_node!(ASTClass::Declare(id, block))
    );
}

#[test]
fn multi_bit_input() {
    let mut b = "declare ok{ input a[2]; }".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let mut interfaces = Vec::new();
    interfaces.push(create_node!(ASTClass::Input(
        create_node!(ASTClass::Identifire("a".to_string())),
        create_node!(ASTClass::Number("2".to_string()))
    )));

    let block = create_node!(ASTClass::Block(interfaces, 1));
    let id = create_node!(ASTClass::Identifire("ok".to_string()));

    assert_eq!(
        p.next_ast().unwrap(),
        create_node!(ASTClass::Declare(id, block))
    );
}

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
        expr
    )));

    let id = create_node!(ASTClass::Identifire("ok".to_string()));
    let block = create_node!(ASTClass::Block(interfaces, 1));

    assert_eq!(
        p.next_ast().unwrap(),
        create_node!(ASTClass::Declare(id, block))
    );
}

#[test]
fn expression_in_width_block_02() {
    let mut b = "declare ok{ input a[OK / 4 * 2]; }".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let left = create_node!(ASTClass::Identifire("OK".to_string()));
    let op = create_node!(ASTClass::Operator(Operator::Slash));
    let right = create_node!(ASTClass::Number("4".to_string()));
    let expr = create_node!(ASTClass::Expression(left, op, right));

    let right2 = create_node!(ASTClass::Number("2".to_string()));

    let op_ast = create_node!(ASTClass::Operator(Operator::Asterisk));
    let expr2 = create_node!(ASTClass::Expression(expr, op_ast, right2));

    let mut interfaces = Vec::new();
    interfaces.push(create_node!(ASTClass::Input(
        create_node!(ASTClass::Identifire("a".to_string())),
        expr2
    )));

    let id = create_node!(ASTClass::Identifire("ok".to_string()));
    let block = create_node!(ASTClass::Block(interfaces, 1));

    assert_eq!(
        p.next_ast().unwrap(),
        create_node!(ASTClass::Declare(id, block))
    );
}

#[test]
fn output_inout() {
    let mut b = "declare ok{ output a[2]; inout b[12];}".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let mut interfaces = Vec::new();
    interfaces.push(create_node!(ASTClass::Output(
        create_node!(ASTClass::Identifire("a".to_string())),
        create_node!(ASTClass::Number("2".to_string()))
    )));

    interfaces.push(create_node!(ASTClass::InOut(
        create_node!(ASTClass::Identifire("b".to_string())),
        create_node!(ASTClass::Number("12".to_string()))
    )));
    assert_eq!(
        p.next_ast().unwrap(),
        create_node!(ASTClass::Declare(
            create_node!(ASTClass::Identifire("ok".to_string())),
            create_node!(ASTClass::Block(interfaces, 1))
        ))
    );
}

#[test]
fn func_in() {
    let mut b = "declare ok{ input a; func_in ok(a);}".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let mut interfaces = Vec::new();
    interfaces.push(create_node!(ASTClass::Input(
        create_node!(ASTClass::Identifire("a".to_string())),
        create_node!(ASTClass::Number("1".to_string()))
    )));

    let args = vec![create_node!(ASTClass::Identifire("a".to_string()))];
    let func = create_node!(ASTClass::FuncIn(
        create_node!(ASTClass::Identifire("ok".to_string())),
        args,
        create_node!(ASTClass::Identifire("".to_string()))
    ));
    interfaces.push(func);

    assert_eq!(
        p.next_ast().unwrap(),
        create_node!(ASTClass::Declare(
            create_node!(ASTClass::Identifire("ok".to_string())),
            create_node!(ASTClass::Block(interfaces, 1))
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
        create_node!(ASTClass::Number("1".to_string()))
    )));
    interfaces.push(create_node!(ASTClass::Output(
        create_node!(ASTClass::Identifire("c".to_string())),
        create_node!(ASTClass::Number("2".to_string()))
    )));
    let args = vec![create_node!(ASTClass::Identifire("a".to_string()))];
    let func = create_node!(ASTClass::FuncIn(
        create_node!(ASTClass::Identifire("ok".to_string())),
        args,
        create_node!(ASTClass::Identifire("c".to_string()))
    ));
    interfaces.push(func);

    assert_eq!(
        p.next_ast().unwrap(),
        create_node!(ASTClass::Declare(
            create_node!(ASTClass::Identifire("ok".to_string())),
            create_node!(ASTClass::Block(interfaces, 1))
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
        create_node!(ASTClass::Number("3".to_string()))
    )));
    interfaces.push(create_node!(ASTClass::Output(
        create_node!(ASTClass::Identifire("c".to_string())),
        create_node!(ASTClass::Number("2".to_string()))
    )));
    let args = vec![create_node!(ASTClass::Identifire("a".to_string()))];
    let func = create_node!(ASTClass::FuncOut(
        create_node!(ASTClass::Identifire("ok".to_string())),
        args,
        create_node!(ASTClass::Identifire("c".to_string()))
    ));
    interfaces.push(func);

    assert_eq!(
        p.next_ast().unwrap(),
        create_node!(ASTClass::Declare(
            create_node!(ASTClass::Identifire("ok".to_string())),
            create_node!(ASTClass::Block(interfaces, 1))
        ))
    );
}

#[test]
fn newline_in_declare_block() {
    let mut b = "declare ok{\n}".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

//     let mut interfaces = vec![create_node!(ASTClass::Newline)];
    let mut interfaces = vec![];
    assert_eq!(
        p.next_ast().unwrap(),
        create_node!(ASTClass::Declare(
            create_node!(ASTClass::Identifire("ok".to_string())),
            create_node!(ASTClass::Block(interfaces, 1))
        ))
    );
}

#[test]
fn declare_03() {
    let mut b = BufReader::new(File::open("nsl_samples/declare_03.nsl").unwrap());
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let mut interfaces = Vec::new();
//     interfaces.push(create_node!(ASTClass::Newline));
    interfaces.push(create_node!(ASTClass::Input(
        create_node!(ASTClass::Identifire("ok".to_string())),
        create_node!(ASTClass::Number("1".to_string()))
    )));
//     interfaces.push(create_node!(ASTClass::Newline));
    interfaces.push(create_node!(ASTClass::Input(
        create_node!(ASTClass::Identifire("ggrks".to_string())),
        create_node!(ASTClass::Number("1".to_string()))
    )));
//     interfaces.push(create_node!(ASTClass::Newline));
    interfaces.push(create_node!(ASTClass::Output(
        create_node!(ASTClass::Identifire("jk".to_string())),
        create_node!(ASTClass::Number("1".to_string()))
    )));
//     interfaces.push(create_node!(ASTClass::Newline));
//     interfaces.push(create_node!(ASTClass::Newline));

    let args1 = vec![create_node!(ASTClass::Identifire("ok".to_string()))];
    let func1 = create_node!(ASTClass::FuncIn(
        create_node!(ASTClass::Identifire("sugoi".to_string())),
        args1,
        create_node!(ASTClass::Identifire("".to_string()))
    ));

    let args2 = vec![create_node!(ASTClass::Identifire("jk".to_string()))];
    let func2 = create_node!(ASTClass::FuncOut(
        create_node!(ASTClass::Identifire("majika".to_string())),
        args2,
        create_node!(ASTClass::Identifire("ggrks".to_string()))
    ));

    interfaces.push(func1);
//     interfaces.push(create_node!(ASTClass::Newline));
    interfaces.push(func2);
//     interfaces.push(create_node!(ASTClass::Newline));

    assert_eq!(
        p.next_ast().unwrap(),
        create_node!(ASTClass::Declare(
            create_node!(ASTClass::Identifire("hel".to_string())),
            create_node!(ASTClass::Block(interfaces, 1))
        ))
    );
}

#[test]
fn include_macro() {
    let mut b = "#include \"hello.h\"\ndeclare ok {}".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let path = create_node!(ASTClass::String("hello.h".to_string()));
    let id = create_node!(ASTClass::Declare(
        create_node!(ASTClass::Identifire("ok".to_string())),
        create_node!(ASTClass::Block(Vec::new(), 1))
    ));
    let include = create_node!(ASTClass::MacroInclude(path));
    assert_eq!(p.next_ast().unwrap(), include);
}

#[test]
fn undef_macro() {
    let mut b = "#undef hello".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let undef = create_node!(ASTClass::MacroUndef(create_node!(
        ASTClass::Identifire("hello".to_string())
    )));
    assert_eq!(p.next_ast().unwrap(), undef);
}

#[test]
fn ifdef_macro() {
    let mut b = "#ifdef hello".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let ifdef = create_node!(ASTClass::MacroIfdef(create_node!(
        ASTClass::Identifire("hello".to_string())
    )));
    assert_eq!(p.next_ast().unwrap(), ifdef);
}

#[test]
fn ifndef_macro() {
    let mut b = "#ifndef hello".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let ifndef = create_node!(ASTClass::MacroIfndef(create_node!(
        ASTClass::Identifire("hello".to_string())
    )));
    assert_eq!(p.next_ast().unwrap(), ifndef);
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
    assert_eq!(p.next_ast().unwrap(), ifndef);
    assert_eq!(p.next_ast().unwrap(), endif);
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
    assert_eq!(p.next_ast().unwrap(), ifndef);
    assert_eq!(p.next_ast().unwrap(), melse);
    assert_eq!(p.next_ast().unwrap(), endif);
}

#[test]
fn define_macro_nl() {
    let mut b = "#define HELLO input ok;\n".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let def_macro = create_node!(ASTClass::MacroDefine(
        create_node!(ASTClass::Identifire("HELLO".to_string())),
        "input ok;".to_string()
    ));
    assert_eq!(p.next_ast().unwrap(), def_macro);
}

#[test]
fn define_macro_eof() {
    let mut b = "#define HELLO input ok;".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let def_macro = create_node!(ASTClass::MacroDefine(
        create_node!(ASTClass::Identifire("HELLO".to_string())),
        "input ok;".to_string()
    ));
    assert_eq!(p.next_ast().unwrap(), def_macro);
}

#[test]
fn define_macro2() {
    // axi4 master interface
    let mut b = "#define AXI4_LITE_MASTER_INTERFACE output awvalid; input awready; output awaddr[AXI_ADDR_WIDTH]; output awprot[3]; output wvalid; input wready; output wdata[AXI_DATA_WIDTH]; output wstrb[AXI_DATA_WIDTH / 8]; input bvalid; output bready; input bresp[2]; output arvalid; input arready; output araddr[AXI_ADDR_WIDTH]; output arprot[3]; input rvalid; output rready; input rdata[AXI_DATA_WIDTH]; input rresp[2];".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let def_macro = create_node!(ASTClass::MacroDefine(
            create_node!(ASTClass::Identifire("AXI4_LITE_MASTER_INTERFACE".to_string())),
            "output awvalid; input awready; output awaddr[ AXI_ADDR_WIDTH ]; output awprot[ 3 ]; output wvalid; input wready; output wdata[ AXI_DATA_WIDTH ]; output wstrb[ AXI_DATA_WIDTH / 8 ]; input bvalid; output bready; input bresp[ 2 ]; output arvalid; input arready; output araddr[ AXI_ADDR_WIDTH ]; output arprot[ 3 ]; input rvalid; output rready; input rdata[ AXI_DATA_WIDTH ]; input rresp[ 2 ];".to_string()));

    assert_eq!(p.next_ast().unwrap(), def_macro);
}

#[test]
fn define_macro3() {
    let mut b = "#define HELLO_ONLY".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let def_macro = create_node!(ASTClass::MacroDefine(
        create_node!(ASTClass::Identifire("HELLO_ONLY".to_string())),
        "".to_string()
    ));
    assert_eq!(p.next_ast().unwrap(), def_macro);
}

#[test]
fn multi_line_comment() {
    let mut b = "/*\ndata lines\n*/".as_bytes();
    let mut l = Lexer::new(&mut b);
    let mut p = Parser::new(&mut l);

    let multi_line = create_node!(ASTClass::CPPStyleComment(
            vec!["".to_string(), "data lines".to_string(), "".to_string()]));

    assert_eq!(p.next_ast().unwrap(), multi_line);
}
