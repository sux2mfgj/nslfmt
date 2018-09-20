use ast::*;
use lexer::*;
use token::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTStatus {
    UnExpectedToken(Token, u32),
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser { lexer: lexer }
    }

    pub fn next_ast(&mut self) -> Result<Box<ASTNode>, ASTStatus> {
        let t = self.lexer.next_token();
        match t.class {
            TokenClass::Identifire(s) => {
                return Ok(Box::new(ASTNode::new(ASTClass::Identifire(s))));
            }
            TokenClass::Number(n) => {
                return Ok(Box::new(ASTNode::new(ASTClass::Number(n))));
            }
            TokenClass::EndOfProgram => {
                return Ok(Box::new(ASTNode::new(ASTClass::EndOfProgram)));
            }
            TokenClass::Symbol(Symbol::Declare) => {
                let id = self.next_ast().unwrap();
                let block = self.next_ast().unwrap();
                return Ok(Box::new(ASTNode::new(ASTClass::Declare(id, block))))
            }
            TokenClass::Symbol(Symbol::Input) => {
                let id = self.next_ast()?;

                match self.lexer.check_next_token().unwrap().class
                {
                    TokenClass::Symbol(Symbol::Semicolon) => {
                        self.lexer.next_token();
                        return Ok(Box::new(ASTNode::new(ASTClass::Input(
                                        id,
                                        Box::new(ASTNode::new(ASTClass::Number("1".to_string())))))));
                    }
                    TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                        let width = self.next_ast()?;
                        let semicolon = self.lexer.next_token();
                        return Ok(Box::new(ASTNode::new(ASTClass::Input(
                                        id,
                                        width))));


                    }
                    _ => {
                        return Err(ASTStatus::UnExpectedToken(self.lexer.next_token(),line!()));
                    }
                }


                let width = self.next_ast().unwrap();
                return Ok(Box::new(ASTNode::new(ASTClass::Input(id, width))));
            }
            TokenClass::Symbol(Symbol::OpeningBrace) => {
                let mut content = Vec::new();
                loop {
                    match self.lexer.check_next_token().unwrap().class {
                        TokenClass::Symbol(Symbol::ClosingBrace) => {
                            self.lexer.next_token();
                            return Ok(Box::new(ASTNode::new(ASTClass::Block(content))));
                        }
                        TokenClass::EndOfProgram => {
                            return Err(ASTStatus::UnExpectedToken(self.lexer.next_token(), line!()));
                        }
                        _ => {
                            let t = self.next_ast()?;
                            content.push(t);
                        }
                    }
                }
            }
            TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                let left_node = self.next_ast()?;
                let center_token = self.lexer.next_token();
                match center_token.class {
                    TokenClass::Operator(op) => {
                        let right_node = self.next_ast()?;
                        return Ok(Box::new(ASTNode::new(
                                    ASTClass::Expression(
                                        left_node,
                                        op,
                                        right_node))));

                    }
                    TokenClass::Symbol(Symbol::RightSquareBracket) => {
                        return Ok(Box::new(ASTNode::new(
                                    ASTClass::WidthBlock(
                                        left_node))));
                    }
                    _ => {
                        return Err(ASTStatus::UnExpectedToken(center_token, line!()));
                    }
                }
            }
            _ => {
                return Err(ASTStatus::UnExpectedToken(t, line!()));
            }
        }
    }
}

#[cfg(test)]
mod parser_test {

    use super::*;
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
    fn unexptected_token() {
        let mut b = "a".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        //TODO
        //assert_eq!(p.next_ast().err(), Some(ASTError::UnExpectedToken(t)));
    }

    #[test]
    fn one_bit_input() {
        let mut b = "declare ok{ input a; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(
                Box::new(ASTNode::new(ASTClass::Input(
                        Box::new(ASTNode::new(ASTClass::Identifire("a".to_string()))),
                        Box::new(ASTNode::new(ASTClass::Number("1".to_string()))))
        )));

        let block = Box::new(ASTNode::new(ASTClass::Block(interfaces)));
        let id = Box::new(ASTNode::new(ASTClass::Identifire("ok".to_string())));
        assert_eq!(
            p.next_ast().unwrap(),
            Box::new(ASTNode::new(ASTClass::Declare(id, block)))
        );
    }

    #[test]
    fn multi_bit_input() {
        /* let mut b = "declare ok{ input a[2]; }".as_bytes(); */
        let mut b = "declare ok{ input a[2]; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(
                Box::new(ASTNode::new(ASTClass::Input(
                        Box::new(ASTNode::new(ASTClass::Identifire("a".to_string()))),
                        Box::new(ASTNode::new(ASTClass::WidthBlock(
                                    Box::new(ASTNode::new(ASTClass::Number("2".to_string())))))))
        )));

        let block = Box::new(ASTNode::new(ASTClass::Block(interfaces)));
        let id = Box::new(ASTNode::new(ASTClass::Identifire("ok".to_string())));
        assert_eq!(
            p.next_ast().unwrap(),
            Box::new(ASTNode::new(ASTClass::Declare(id, block)))
        );
    }

    /*
    #[test]
    fn expression_on_width_block() {
        let mut b = "declare ok{ input a[OK / 2]; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

    }
    */

    /*
    #[test]
    fn output_inout() {
        let mut b = "declare ok{ output a[2]; inout b[12];}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(ASTNode::new(ASTClass::Output(
            "a".to_string(),
            "2".to_string(),
        )));
        interfaces.push(ASTNode::new(ASTClass::InOut(
            "b".to_string(),
            "12".to_string(),
        )));
        assert_eq!(
            p.next_ast().unwrap(),
            ASTNode::new(ASTClass::Declare("ok".to_string(), interfaces))
        );
    }

    #[test]
    fn func_in() {
        let mut b = "declare ok{ input a; func_in ok(a);}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(ASTNode::new(ASTClass::Input(
            "a".to_string(),
            "1".to_string(),
        )));
        let mut arg_vec = Vec::new();
        arg_vec.push("a".to_string());
        interfaces.push(ASTNode::new(ASTClass::FuncIn(
            "ok".to_string(),
            arg_vec,
            "".to_string(),
        )));
        assert_eq!(
            p.next_ast().unwrap(),
            ASTNode::new(ASTClass::Declare("ok".to_string(), interfaces))
        );
    }

    #[test]
    fn func_in_return() {
        let mut b = "declare ok{ input a; output c[2]; func_in ok(a): c;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(ASTNode::new(ASTClass::Input(
            "a".to_string(),
            "1".to_string(),
        )));
        interfaces.push(ASTNode::new(ASTClass::Output(
            "c".to_string(),
            "2".to_string(),
        )));
        let mut arg_vec = Vec::new();
        arg_vec.push("a".to_string());
        interfaces.push(ASTNode::new(ASTClass::FuncIn(
            "ok".to_string(),
            arg_vec,
            "c".to_string(),
        )));
        assert_eq!(
            p.next_ast().unwrap(),
            ASTNode::new(ASTClass::Declare("ok".to_string(), interfaces))
        );
    }

    #[test]
    fn func_out_return() {
        let mut b = "declare ok{ input a[3]; output c[2]; func_out ok(a): c;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(ASTNode::new(ASTClass::Input(
            "a".to_string(),
            "3".to_string(),
        )));
        interfaces.push(ASTNode::new(ASTClass::Output(
            "c".to_string(),
            "2".to_string(),
        )));
        let mut arg_vec = Vec::new();
        arg_vec.push("a".to_string());
        interfaces.push(ASTNode::new(ASTClass::FuncOut(
            "ok".to_string(),
            arg_vec,
            "c".to_string(),
        )));
        assert_eq!(
            p.next_ast().unwrap(),
            ASTNode::new(ASTClass::Declare("ok".to_string(), interfaces))
        );
    }

    #[test]
    fn declare_03() {
        let mut b = BufReader::new(File::open("test_code/declare_03.nsl").unwrap());
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(ASTNode::new(ASTClass::Input(
            "ok".to_string(),
            "1".to_string(),
        )));
        interfaces.push(ASTNode::new(ASTClass::Input(
            "ggrks".to_string(),
            "1".to_string(),
        )));
        interfaces.push(ASTNode::new(ASTClass::Output(
            "jk".to_string(),
            "1".to_string(),
        )));
        let mut f1_arg_vec = Vec::new();
        f1_arg_vec.push("ok".to_string());
        interfaces.push(ASTNode::new(ASTClass::FuncIn(
            "sugoi".to_string(),
            f1_arg_vec,
            "".to_string(),
        )));

        let mut f2_arg_vec = Vec::new();
        f2_arg_vec.push("jk".to_string());
        interfaces.push(ASTNode::new(ASTClass::FuncOut(
            "majika".to_string(),
            f2_arg_vec,
            "ggrks".to_string(),
        )));

        assert_eq!(
            p.next_ast().unwrap(),
            ASTNode::new(ASTClass::Declare("hel".to_string(), interfaces))
        );
    }

    #[test]
    fn include_macro() {
        let mut b = "#include \"hello.h\"\ndeclare ok {}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::MacroInclude("\"hello.h\"".to_string())));
        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::Declare("ok".to_string(), Vec::new())));
        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::EndOfProgram));
    }

    #[test]
    fn undef_macro() {
        let mut b = "#undef hello".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::MacroUndef("hello".to_string())));
        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::EndOfProgram));
    }

    #[test]
    fn ifdef_macro() {
        let mut b = "#ifdef hello".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::MacroIfdef("hello".to_string())));
        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::EndOfProgram));
    }

    #[test]
    fn ifndef_macro() {
        let mut b = "#ifndef hello".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::MacroIfndef("hello".to_string())));
        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::EndOfProgram));
    }

    #[test]
    fn else_macro() {
        let mut b = "#else".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::MacroElse));
        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::EndOfProgram));
    }

    #[test]
    fn endif_macro() {
        let mut b = "#endif".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::MacroEndif));
        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::EndOfProgram));
    }

    #[test]
    fn define_macro() {
        let mut b = "#define HELLO input ok;".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut define_arg = Vec::new();
        define_arg.push(ASTNode::new(
                    ASTClass::Input(
                        "ok".to_string(),
                        "1".to_string())));
        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::MacroDefine(
                        "HELLO".to_string(),
                        define_arg)));
        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::EndOfProgram));
    }

    #[test]
    fn division() {
        let mut b = "define OK input test[10 / 2];";
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut define_arg = Vec::new();
        define_arg.push(
                ASTNode::new(
                    ASTClass::Input(
                        "test".to_string(),
                        "1".to_string())));
        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::MacroDefine(
                        "OK".to_string(),
                        define_arg)));
        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::EndOfProgram));
    }

    /*
    #[test]
    fn define_macro2() {
        // axi4 master interface
        let mut b = "#define AXI4_LITE_MASTER_INTERFACE output awvalid; input awready; output awaddr[AXI_ADDR_WIDTH]; output awprot[3]; output wvalid; input wready; output wdata[AXI_DATA_WIDTH]; output wstrb[AXI_DATA_WIDTH / 8]; input bvalid; output bready; input bresp[2]; output arvalid; input arready; output araddr[AXI_ADDR_WIDTH]; output arprot[3]; input rvalid; output rready; input rdata[AXI_DATA_WIDTH]; input rresp[2];".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut define_arg = Vec::new();
        define_arg.push(ASTNode::new(
                    ASTClass::Input(
                        "ok".to_string(),
                        "1".to_string())));

        let mut define_arg = Vec::new();
        define_arg.push(ASTNode::new(
                    ASTClass::Output(
                        "awvalid".to_string(),
                        "1".to_string())));
        define_arg.push(ASTNode::new(
                    ASTClass::Input(
                        "awready".to_string(),
                       "1".to_string())));
        define_arg.push(ASTNode::new(
                    ASTClass::Output(
                        "awaddr".to_string(),
                        "AXI_ADDR_WIDTH".to_string())));


        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::MacroDefine(
                        "AXI4_LITE_MASTER_INTERFACE".to_string(),
                        define_arg)));
        assert_eq!(
                p.next_ast().unwrap(),
                ASTNode::new(ASTClass::EndOfProgram));
    }
    */
    */
}
