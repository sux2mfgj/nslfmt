use ast::*;
use lexer::*;
use token::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTError {
    UnExpectedToken(Token),
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser { lexer: lexer }
    }

    pub fn next_ast(&mut self) -> Result<ASTNode, ASTError> {
        let t = self.lexer.get_next_token();
        match t.class {
            TokenClass::Symbol(Symbol::Declare) => {
                return self.generate_declare_ast();
            }
            TokenClass::Symbol(Symbol::Sharp) => {
                return self.generate_macro_ast();
            }
            //TokenClass::Symbol(Symbol::Module) => {
            //}
            TokenClass::Symbol(Symbol::Input) => {
                let id = self.get_id().unwrap();
                let number = self.get_width().unwrap();
                return Ok(ASTNode::new(ASTClass::Input(id, number)));
            }
            TokenClass::Symbol(Symbol::Output) => {
                let id = self.get_id().unwrap();
                let number = self.get_width().unwrap();
                return Ok(ASTNode::new(ASTClass::Output(id, number)));
            }
            TokenClass::Symbol(Symbol::InOut) => {
                let id = self.get_id().unwrap();
                let number = self.get_width().unwrap();
                return Ok(ASTNode::new(ASTClass::InOut(id, number)));
            }
            TokenClass::Symbol(Symbol::FuncIn) => {
                let id = self.get_id().unwrap();
                let args = self.get_arguments().unwrap();
                let return_port = self.get_return_port().unwrap();
                return Ok(ASTNode::new(
                        ASTClass::FuncIn(id, args, return_port)));
            }
            TokenClass::Symbol(Symbol::FuncOut) => {
                let id = self.get_id().unwrap();
                let args = self.get_arguments().unwrap();
                let return_port = self.get_return_port().unwrap();
                return Ok(ASTNode::new(
                        ASTClass::FuncOut(id, args, return_port)));
            }
            TokenClass::Newline => {
                return self.next_ast();
            }
            TokenClass::EndOfProgram => {
                return Ok(ASTNode::new(ASTClass::EndOfProgram));
            }
            _ => Err(ASTError::UnExpectedToken(t)),
        }
    }

    fn generate_macro_ast(&mut self) -> Result<ASTNode, ASTError> {
        let t = self.lexer.get_next_token();
        match t.class {
            TokenClass::Macro(Macro::Include) => {
                if let Ok(s) = self.get_string_with_dquote() {
                    return Ok(ASTNode::new(ASTClass::MacroInclude(s)));
                }
                else {
                    return Err(ASTError::UnExpectedToken(t));
                }
            }
            TokenClass::Macro(Macro::Undef) => {
                match self.lexer.get_next_token().class {
                    TokenClass::Identifire(s) => {
                        return Ok(ASTNode::new(ASTClass::MacroUndef(s)));
                    }
                    _ => {
                        return Err(ASTError::UnExpectedToken(t));
                    }
                }
            }
            TokenClass::Macro(Macro::Ifdef) => {
                match self.lexer.get_next_token().class {
                    TokenClass::Identifire(s) => {
                        return Ok(ASTNode::new(ASTClass::MacroIfdef(s)));
                    }
                    _ => {
                        return Err(ASTError::UnExpectedToken(t));
                    }
                }
            }
            TokenClass::Macro(Macro::Ifndef) => {
                match self.lexer.get_next_token().class {
                    TokenClass::Identifire(s) => {
                        return Ok(ASTNode::new(ASTClass::MacroIfndef(s)));
                    }
                    _ => {
                        return Err(ASTError::UnExpectedToken(t));
                    }
                }
            }
            TokenClass::Macro(Macro::Else) => {
                return Ok(ASTNode::new(ASTClass::MacroElse));
            }
            TokenClass::Macro(Macro::Endif) => {
                return Ok(ASTNode::new(ASTClass::MacroEndif));
            }
            TokenClass::Macro(Macro::Define) => {
                let id = self.get_id().unwrap();
                let mut define_arg = Vec::new();
                loop {
                    match self.lexer.check_next_token().unwrap().class
                    {
                        TokenClass::Newline | TokenClass::EndOfProgram =>
                        {
                            self.lexer.get_next_token();
                            return Ok(ASTNode::new(
                                        ASTClass::MacroDefine(
                                            id, define_arg)));

                        }
                        _ => {
                            define_arg.push(self.next_ast().unwrap());
                        }
                    }
                }
            }
            //TODO
            _ => {
                return Err(ASTError::UnExpectedToken(t));
            }
        }
    }

    fn generate_declare_ast(&mut self) -> Result<ASTNode, ASTError> {
        let root_node: ASTNode;
        let d_name_token = self.lexer.get_next_token();
        let mut interfaces = Vec::new();

        let open_brace = self.lexer.get_next_token();
        if let TokenClass::Symbol(Symbol::OpeningBrace) = open_brace.class {
        } else {
            return Err(ASTError::UnExpectedToken(open_brace));
        }

        loop {
            let t = self.lexer.get_next_token();
            match t.class {
                TokenClass::Symbol(Symbol::ClosingBrace) => {
                    break;
                }
                TokenClass::Symbol(Symbol::Input) => {
                    let id = self.get_id().unwrap();
                    let number = self.get_width().unwrap();
                    let node = ASTNode::new(ASTClass::Input(id, number));
                    interfaces.push(node);
                }
                TokenClass::Symbol(Symbol::Output) => {
                    let id = self.get_id().unwrap();
                    let number = self.get_width().unwrap();
                    let node = ASTNode::new(ASTClass::Output(id, number));
                    interfaces.push(node);
                }
                TokenClass::Symbol(Symbol::InOut) => {
                    let id = self.get_id().unwrap();
                    let number = self.get_width().unwrap();
                    let node = ASTNode::new(ASTClass::InOut(id, number));
                    interfaces.push(node);
                }
                TokenClass::Symbol(Symbol::FuncIn) => {
                    let id = self.get_id().unwrap();
                    let args = self.get_arguments().unwrap();
                    let return_port = self.get_return_port().unwrap();
                    let node = ASTNode::new(ASTClass::FuncIn(id, args, return_port));
                    interfaces.push(node);
                }
                TokenClass::Symbol(Symbol::FuncOut) => {
                    let id = self.get_id().unwrap();
                    let args = self.get_arguments().unwrap();
                    let return_port = self.get_return_port().unwrap();
                    let node = ASTNode::new(ASTClass::FuncOut(id, args, return_port));
                    interfaces.push(node);
                }
                _ => {
                    return Err(ASTError::UnExpectedToken(t));
                }
            }
        }
        if let TokenClass::Identifire(name) = d_name_token.class {
            let ast_class = ASTClass::Declare(name, interfaces);
            root_node = ASTNode::new(ast_class);
            return Ok(root_node);
        } else {
            return Err(ASTError::UnExpectedToken(d_name_token));
        }
    }

    fn get_string_with_dquote(&mut self) -> Result<String, ASTError> {
        let mut file_path = "\"".to_string();
        let dquote1 = self.lexer.get_next_token();
        if let TokenClass::Symbol(Symbol::DoubleQuote) = dquote1.class {}
        else {
            return Err(ASTError::UnExpectedToken(dquote1));
        }
        loop {
            let t = self.lexer.get_next_token();
            match t.class {
                TokenClass::Identifire(id) => {
                    file_path.push_str(&id);
                }
                TokenClass::Symbol(Symbol::Dot) => {
                    file_path.push_str(".");
                }
                TokenClass::Symbol(Symbol::DoubleQuote) => {
                    file_path.push_str("\"");
                    return Ok(file_path);
                }
                _ => {
                    return Err(ASTError::UnExpectedToken(t));
                }
            }
        }
    }

    fn get_id(&mut self) -> Result<String, ASTError> {
        let id_token = self.lexer.get_next_token();
        if let TokenClass::Identifire(id) = id_token.class {
            return Ok(id);
        } else {
            return Err(ASTError::UnExpectedToken(id_token));
        }
    }

    fn get_semicolon(&mut self) -> Option<ASTError> {
        let token = self.lexer.get_next_token();
        if let TokenClass::Symbol(Symbol::Semicolon) = token.class {
            return None;
        } else {
            return Some(ASTError::UnExpectedToken(token));
        }
    }

    fn get_width(&mut self) -> Result<String, ASTError> {
        let width_token = self.lexer.get_next_token();
        match width_token.class {
            TokenClass::Symbol(Symbol::Semicolon) => {
                return Ok("1".to_string());
            }
            TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                return self.get_number();
            }
            _ => {
                return Err(ASTError::UnExpectedToken(width_token));
            }
        }
    }

    fn get_number(&mut self) -> Result<String, ASTError> {
        let num_token = self.lexer.get_next_token();

        let right_bracket_token = self.lexer.get_next_token();
        if let TokenClass::Symbol(Symbol::RightSquareBracket) = right_bracket_token.class
        {
        } else {
            return Err(ASTError::UnExpectedToken(right_bracket_token));
        }

        let semicolon_token = self.lexer.get_next_token();
        if let TokenClass::Symbol(Symbol::Semicolon) = semicolon_token.class {
        } else {
            return Err(ASTError::UnExpectedToken(semicolon_token));
        }

        match num_token.class {
            TokenClass::Number(n1) => {
                return Ok(n1);
            }
            TokenClass::Identifire(n2) => {
                return Ok(n2);
            }
            _ => {
                return Err(ASTError::UnExpectedToken(num_token));
            }
        }
    }

    fn get_arguments(&mut self) -> Result<Vec<String>, ASTError> {
        let left_paren = self.lexer.get_next_token();
        if let TokenClass::Symbol(Symbol::LeftParen) = left_paren.class {
        } else {
            return Err(ASTError::UnExpectedToken(left_paren));
        }

        let mut args = Vec::new();

        loop {
            let t = self.lexer.get_next_token();
            match t.class {
                TokenClass::Identifire(id) => {
                    args.push(id);
                }
                TokenClass::Symbol(Symbol::RightParen) => {
                    return Ok(args);
                }
                TokenClass::Symbol(Symbol::Comma) => {
                    continue;
                }
                _ => {
                    return Err(ASTError::UnExpectedToken(t));
                }
            }
        }
    }

    fn get_return_port(&mut self) -> Result<String, ASTError> {
        let colon_of_semicolon = self.lexer.get_next_token();

        if let TokenClass::Symbol(Symbol::Semicolon) = colon_of_semicolon.class {
            return Ok("".to_string());
        }

        if let TokenClass::Symbol(Symbol::Colon) = colon_of_semicolon.class {
            let id = self.get_id();
            if let Some(e) = self.get_semicolon() {
                return Err(e);
            } else {
                return id;
            }
        } else {
            Err(ASTError::UnExpectedToken(colon_of_semicolon))
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
    fn number() {
        let mut b = "declare ok{ input a[2]; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(ASTNode::new(ASTClass::Input(
            "a".to_string(),
            "2".to_string(),
        )));
        assert_eq!(
            p.next_ast().unwrap(),
            ASTNode::new(ASTClass::Declare("ok".to_string(), interfaces))
        );
    }

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
}
