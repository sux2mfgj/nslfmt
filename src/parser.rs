use lexer::*;
use token::*;
use ast::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTError {
    EndOfProgram,
    UnExpectedToken,
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer: lexer,
        }
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
                Err(ASTError::EndOfProgram)
            }
            _ => {
                Err(ASTError::UnExpectedToken)
            }
        }
    }

    fn generate_declare_ast(&mut self, token: Token) -> Result<ASTNode, ASTError> {
        let root_node : ASTNode;
        let d_name_token = self.lexer.get_next_token();
        let mut io_vec = Vec::new();
        let mut func_vec = Vec::new();

        let open_brace = self.lexer.get_next_token();
        if let TokenClass::Symbol(Symbol::OpeningBrace) = open_brace.class {
        } else
        {
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
                _ => {
                    return Err(ASTError::UnExpectedToken);
                }
            }
        }
        if let TokenClass::Identifire(name) = d_name_token.class {
            let ast_class = ASTClass::Declare(name, io_vec, func_vec);
            root_node = ASTNode::new(ast_class);
            return Ok(root_node);
        } else
        {
            return Err(ASTError::UnExpectedToken);
        }
    }

    fn get_id(&mut self) -> Result<String, ASTError> {
        let id_token = self.lexer.get_next_token();
        if let TokenClass::Identifire(id) = id_token.class {
            return Ok(id);
        }
        else
        {
            return Err(ASTError::UnExpectedToken);
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
        if let TokenClass::Symbol(Symbol::RightSquareBracket) = right_bracket_token.class {
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
        }
        else {
            return Err(ASTError::UnExpectedToken);
        }
    }
}

#[cfg(test)]
mod parser_test {

    use super::*;

    #[test]
    fn end_of_program() {
        let mut b = "".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        assert_eq!(p.next_ast().err(), Some(ASTError::EndOfProgram));
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
        io_vec.push(ASTNode::new(ASTClass::Input("a".to_string(), "2".to_string())));
        let func_vec = Vec::new();
        assert_eq!(p.next_ast().unwrap(),
                   ASTNode::new(ASTClass::Declare("ok".to_string(), io_vec, func_vec)));
    }

    #[test]
    fn output_inout() {
        let mut b = "declare ok{ output a[2]; inout b[12];}".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut io_vec = Vec::new();
        io_vec.push(ASTNode::new(ASTClass::Output("a".to_string(), "2".to_string())));
        io_vec.push(ASTNode::new(ASTClass::InOut("b".to_string(), "12".to_string())));
        let func_vec = Vec::new();
        assert_eq!(p.next_ast().unwrap(),
                   ASTNode::new(ASTClass::Declare("ok".to_string(), io_vec, func_vec)));
    }
}
