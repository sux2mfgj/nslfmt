use ast::*;
use lexer::*;
use token::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTError {
    //EndOfProgram,
    UnExpectedToken,
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
                return self.generate_declare_ast(t);
            }
            //TokenClass::Symbol(Symbol::Module) => {
            //}
            TokenClass::EndOfProgram => {
                return Ok(ASTNode::new(ASTClass::EndOfProgram));
            }
            _ => Err(ASTError::UnExpectedToken),
        }
    }

    fn generate_declare_ast(&mut self, _token: Token) -> Result<ASTNode, ASTError> {
        let root_node: ASTNode;
        let d_name_token = self.lexer.get_next_token();
        let mut io_vec = Vec::new();
        let mut func_vec = Vec::new();

        let open_brace = self.lexer.get_next_token();
        if let TokenClass::Symbol(Symbol::OpeningBrace) = open_brace.class {
        } else {
            return Err(ASTError::UnExpectedToken);
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
                    io_vec.push(node);
                }
                TokenClass::Symbol(Symbol::Output) => {
                    let id = self.get_id().unwrap();
                    let number = self.get_width().unwrap();
                    let node = ASTNode::new(ASTClass::Output(id, number));
                    io_vec.push(node);
                }
                TokenClass::Symbol(Symbol::InOut) => {
                    let id = self.get_id().unwrap();
                    let number = self.get_width().unwrap();
                    let node = ASTNode::new(ASTClass::InOut(id, number));
                    io_vec.push(node);
                }
                TokenClass::Symbol(Symbol::FuncIn) => {
                    let id = self.get_id().unwrap();
                    let args = self.get_arguments().unwrap();
                    let return_port = self.get_return_port().unwrap();
                    let node = ASTNode::new(ASTClass::FuncIn(id, args, return_port));
                    func_vec.push(node);
                }
                TokenClass::Symbol(Symbol::FuncOut) => {
                    let id = self.get_id().unwrap();
                    let args = self.get_arguments().unwrap();
                    let return_port = self.get_return_port().unwrap();
                    let node = ASTNode::new(ASTClass::FuncOut(id, args, return_port));
                    func_vec.push(node);
                }
                _ => {
                    return Err(ASTError::UnExpectedToken);
                }
            }
        }
        if let TokenClass::Identifire(name) = d_name_token.class {
            let ast_class = ASTClass::Declare(name, io_vec, func_vec);
            root_node = ASTNode::new(ast_class);
            return Ok(root_node);
        } else {
            return Err(ASTError::UnExpectedToken);
        }
    }

    fn get_id(&mut self) -> Result<String, ASTError> {
        let id_token = self.lexer.get_next_token();
        if let TokenClass::Identifire(id) = id_token.class {
            return Ok(id);
        } else {
            return Err(ASTError::UnExpectedToken);
        }
    }

    fn get_semicolon(&mut self) -> Option<ASTError> {
        let token = self.lexer.get_next_token();
        if let TokenClass::Symbol(Symbol::Semicolon) = token.class {
            return None;
        } else {
            return Some(ASTError::UnExpectedToken);
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
                return Err(ASTError::UnExpectedToken);
            }
        }
    }

    fn get_number(&mut self) -> Result<String, ASTError> {
        let num_token = self.lexer.get_next_token();

        let right_bracket_token = self.lexer.get_next_token();
        if let TokenClass::Symbol(Symbol::RightSquareBracket) = right_bracket_token.class
        {
        } else {
            return Err(ASTError::UnExpectedToken);
        }

        let semicolon_token = self.lexer.get_next_token();
        if let TokenClass::Symbol(Symbol::Semicolon) = semicolon_token.class {
        } else {
            return Err(ASTError::UnExpectedToken);
        }

        if let TokenClass::Number(num) = num_token.class {
            return Ok(num);
        } else {
            return Err(ASTError::UnExpectedToken);
        }
    }

    fn get_arguments(&mut self) -> Result<Vec<String>, ASTError> {
        let left_paren = self.lexer.get_next_token();
        if let TokenClass::Symbol(Symbol::LeftParen) = left_paren.class {
        } else {
            return Err(ASTError::UnExpectedToken);
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
                    return Err(ASTError::UnExpectedToken);
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
            Err(ASTError::UnExpectedToken)
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

        assert_eq!(p.next_ast().err(), Some(ASTError::UnExpectedToken));
    }

    #[test]
    fn number() {
        let mut b = "declare ok{ input a[2]; }".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut io_vec = Vec::new();
        io_vec.push(ASTNode::new(ASTClass::Input(
            "a".to_string(),
            "2".to_string(),
        )));
        let func_vec = Vec::new();
        assert_eq!(
            p.next_ast().unwrap(),
            ASTNode::new(ASTClass::Declare("ok".to_string(), io_vec, func_vec))
        );
    }

    #[test]
    fn output_inout() {
        let mut b = "declare ok{ output a[2]; inout b[12];}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut io_vec = Vec::new();
        io_vec.push(ASTNode::new(ASTClass::Output(
            "a".to_string(),
            "2".to_string(),
        )));
        io_vec.push(ASTNode::new(ASTClass::InOut(
            "b".to_string(),
            "12".to_string(),
        )));
        let func_vec = Vec::new();
        assert_eq!(
            p.next_ast().unwrap(),
            ASTNode::new(ASTClass::Declare("ok".to_string(), io_vec, func_vec))
        );
    }

    #[test]
    fn func_in() {
        let mut b = "declare ok{ input a; func_in ok(a);}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut io_vec = Vec::new();
        io_vec.push(ASTNode::new(ASTClass::Input(
            "a".to_string(),
            "1".to_string(),
        )));
        let mut func_vec = Vec::new();
        let mut arg_vec = Vec::new();
        arg_vec.push("a".to_string());
        func_vec.push(ASTNode::new(ASTClass::FuncIn(
            "ok".to_string(),
            arg_vec,
            "".to_string(),
        )));
        assert_eq!(
            p.next_ast().unwrap(),
            ASTNode::new(ASTClass::Declare("ok".to_string(), io_vec, func_vec))
        );
    }

    #[test]
    fn func_in_return() {
        let mut b = "declare ok{ input a; output c[2]; func_in ok(a): c;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut io_vec = Vec::new();
        io_vec.push(ASTNode::new(ASTClass::Input(
            "a".to_string(),
            "1".to_string(),
        )));
        io_vec.push(ASTNode::new(ASTClass::Output(
            "c".to_string(),
            "2".to_string(),
        )));
        let mut func_vec = Vec::new();
        let mut arg_vec = Vec::new();
        arg_vec.push("a".to_string());
        func_vec.push(ASTNode::new(ASTClass::FuncIn(
            "ok".to_string(),
            arg_vec,
            "c".to_string(),
        )));
        assert_eq!(
            p.next_ast().unwrap(),
            ASTNode::new(ASTClass::Declare("ok".to_string(), io_vec, func_vec))
        );
    }

    #[test]
    fn func_out_return() {
        let mut b = "declare ok{ input a[3]; output c[2]; func_out ok(a): c;}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut io_vec = Vec::new();
        io_vec.push(ASTNode::new(ASTClass::Input(
            "a".to_string(),
            "3".to_string(),
        )));
        io_vec.push(ASTNode::new(ASTClass::Output(
            "c".to_string(),
            "2".to_string(),
        )));
        let mut func_vec = Vec::new();
        let mut arg_vec = Vec::new();
        arg_vec.push("a".to_string());
        func_vec.push(ASTNode::new(ASTClass::FuncOut(
            "ok".to_string(),
            arg_vec,
            "c".to_string(),
        )));
        assert_eq!(
            p.next_ast().unwrap(),
            ASTNode::new(ASTClass::Declare("ok".to_string(), io_vec, func_vec))
        );
    }

    #[test]
    fn declare_03() {
        let mut b = BufReader::new(File::open("test_code/declare_03.nsl").unwrap());
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut io_vec = Vec::new();
        io_vec.push(ASTNode::new(ASTClass::Input(
            "ok".to_string(),
            "1".to_string(),
        )));
        io_vec.push(ASTNode::new(ASTClass::Input(
            "ggrks".to_string(),
            "1".to_string(),
        )));
        io_vec.push(ASTNode::new(ASTClass::Output(
            "jk".to_string(),
            "1".to_string(),
        )));
        let mut func_vec = Vec::new();
        let mut f1_arg_vec = Vec::new();
        f1_arg_vec.push("ok".to_string());
        func_vec.push(ASTNode::new(ASTClass::FuncIn(
            "sugoi".to_string(),
            f1_arg_vec,
            "".to_string(),
        )));

        let mut f2_arg_vec = Vec::new();
        f2_arg_vec.push("jk".to_string());
        func_vec.push(ASTNode::new(ASTClass::FuncOut(
            "majika".to_string(),
            f2_arg_vec,
            "ggrks".to_string(),
        )));

        assert_eq!(
            p.next_ast().unwrap(),
            ASTNode::new(ASTClass::Declare("hel".to_string(), io_vec, func_vec))
        );
    }
}
