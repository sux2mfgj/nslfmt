
use ast::*;
use lexer::*;
use token::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTError {
    UnExpectedToken(Token, u32),
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    number_of_next: usize,
}

#[macro_export]
macro_rules! create_node {
    ($n:expr) => {
        Box::new(ASTNode::new($n));
    };
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer: lexer,
            number_of_next: 1,
        }
    }

    pub fn next_ast_enable_newline(&mut self) -> Result<Box<ASTNode>, ASTError> {
        match self.lexer.check_next_token(false).class {
            TokenClass::Newline => {
                self.lexer.next_token(false);
                return Ok(create_node!(ASTClass::Newline));
            }
            _ => {
                return self.next_ast();
            }
        }
    }

    pub fn next_ast(&mut self) -> Result<Box<ASTNode>, ASTError> {
        let t = self.lexer.next_token(true);
        match t.class {
            TokenClass::Identifire(s) => {
                return Ok(create_node!(ASTClass::Identifire(s)));
            }
            TokenClass::Number(n) => {
                return Ok(create_node!(ASTClass::Number(n)));
            }
            TokenClass::String(s) => {
                return Ok(create_node!(ASTClass::String(s)));
            }
            TokenClass::EndOfProgram => {
                return Ok(create_node!(ASTClass::EndOfProgram));
            }
            TokenClass::Symbol(Symbol::Declare) => {
                let id = self.next_ast()?;
                let block = self.next_ast()?;
                return Ok(create_node!(ASTClass::Declare(id, block)));
            }
            TokenClass::Symbol(Symbol::Input) => {
                let id = self.next_ast()?;

                match self.lexer.check_next_token(true).class {
                    TokenClass::Symbol(Symbol::Semicolon) => {
                        self.lexer.next_token(true);
                        return Ok(create_node!(ASTClass::Input(
                            id,
                            create_node!(ASTClass::Number("1".to_string()))
                        )));
                    }
                    TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                        let width = self.next_ast()?;
                        let _semicolon = self.lexer.next_token(true);
                        return Ok(create_node!(ASTClass::Input(id, width)));
                    }
                    _ => {
                        return Err(ASTError::UnExpectedToken(
                            self.lexer.next_token(true),
                            line!(),
                        ));
                    }
                }
            }
            //TODO use macro. almost same with Input and InOut.
            TokenClass::Symbol(Symbol::Output) => {
                let id = self.next_ast()?;

                match self.lexer.check_next_token(true).class {
                    TokenClass::Symbol(Symbol::Semicolon) => {
                        self.lexer.next_token(true);
                        return Ok(create_node!(ASTClass::Output(
                            id,
                            create_node!(ASTClass::Number("1".to_string()))
                        )));
                    }
                    TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                        let width = self.next_ast()?;
                        let _semicolon = self.lexer.next_token(true);
                        return Ok(create_node!(ASTClass::Output(id, width)));
                    }
                    _ => {
                        return Err(ASTError::UnExpectedToken(
                            self.lexer.next_token(true),
                            line!(),
                        ));
                    }
                }
            }
            //TODO use macro
            TokenClass::Symbol(Symbol::InOut) => {
                let id = self.next_ast()?;

                match self.lexer.check_next_token(true).class {
                    TokenClass::Symbol(Symbol::Semicolon) => {
                        self.lexer.next_token(true);
                        return Ok(create_node!(ASTClass::InOut(
                            id,
                            create_node!(ASTClass::Number("1".to_string()))
                        )));
                    }
                    TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                        let width = self.next_ast()?;
                        let _semicolon = self.lexer.next_token(true);
                        return Ok(create_node!(ASTClass::InOut(id, width)));
                    }
                    _ => {
                        return Err(ASTError::UnExpectedToken(
                            self.lexer.next_token(true),
                            line!(),
                        ));
                    }
                }
            }
            TokenClass::Symbol(Symbol::FuncIn) => {
                let id = self.next_ast()?;
                let args = self.generate_args_vec()?;

                if let TokenClass::Symbol(Symbol::Colon) = self.lexer.next_token(true).class {
                    let out_token = self.lexer.next_token(true);
                    if let TokenClass::Identifire(s) = out_token.class {
                        let _semicolon = self.lexer.next_token(true);
                        let return_node = create_node!(ASTClass::Identifire(s));
                        return Ok(create_node!(ASTClass::FuncIn(id, args, return_node)));
                    } else {
                        return Err(ASTError::UnExpectedToken(out_token, line!()));
                    }
                } else {
                    return Ok(create_node!(ASTClass::FuncIn(
                        id,
                        args,
                        create_node!(ASTClass::Identifire("".to_string()))
                    )));
                }
            }
            TokenClass::Symbol(Symbol::FuncOut) => {
                let id = self.next_ast()?;
                let args = self.generate_args_vec()?;

                if let TokenClass::Symbol(Symbol::Colon) = self.lexer.next_token(true).class {
                    let out_token = self.lexer.next_token(true);
                    if let TokenClass::Identifire(s) = out_token.class {
                        let _semicolon = self.lexer.next_token(true);
                        let return_node = create_node!(ASTClass::Identifire(s));
                        return Ok(create_node!(ASTClass::FuncOut(id, args, return_node)));
                    } else {
                        return Err(ASTError::UnExpectedToken(out_token, line!()));
                    }
                } else {
                    return Ok(create_node!(ASTClass::FuncOut(
                        id,
                        args,
                        create_node!(ASTClass::Identifire("".to_string()))
                    )));
                }
            }
            TokenClass::Symbol(Symbol::OpeningBrace) => {
                let mut content = Vec::new();
                self.number_of_next += 1;
                loop {
                    let next_t = self.lexer.check_next_token(true);
//                     match self.lexer.check_next_token(true).class {
                    match next_t.class {
                        TokenClass::Symbol(Symbol::ClosingBrace) => {
                            self.lexer.next_token(true);
                            self.number_of_next -= 1;
                            return Ok(create_node!(ASTClass::Block(
                                content,
                                self.number_of_next
                            )));
                        }
                        TokenClass::EndOfProgram => {
                            return Err(ASTError::UnExpectedToken(
                                self.lexer.next_token(true),
                                line!(),
                            ));
                        }
                        _ => {
                            let t = self.next_ast_enable_newline()?;
                            content.push(t);
                        }
                    }
                }
            }
            TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                let left = self.next_ast()?;
                let expr = self.create_expression(left)?;
                let next_token = self.lexer.next_token(true);
                match next_token.class {
                    TokenClass::Symbol(Symbol::RightSquareBracket) => {
                        return Ok(expr);
                    }
                    _ => {
                        return Err(ASTError::UnExpectedToken(next_token, line!()));
                    }
                }
            }
            TokenClass::Symbol(Symbol::LeftParen) => {
                let first = self.next_ast()?;
                let expr = self.create_expression(first)?;
                let next_token = self.lexer.next_token(true);
                match next_token.class {
                    TokenClass::Symbol(Symbol::RightParen) => {
                        return Ok(expr);
                    }
                    _ => {
                        panic!("unexptected token: {}", next_token);
                    }
                }
            }
            TokenClass::Symbol(Symbol::Sharp) => {
                return self.generate_macro_astnode();
            }
            TokenClass::Operator(op) => {
                return Ok(create_node!(ASTClass::Operator(op)));
            }
            TokenClass::CStyleComment(line) => {
                return Ok(create_node!(ASTClass::CStyleComment(line)));
            }
            TokenClass::CPPStyleComment(list) => {
                return Ok(create_node!(ASTClass::CPPStyleComment(list)));
            }
            _ => {
                panic!("unexptected token: {}", t);
            }
        }
    }

    fn generate_macro_astnode(&mut self) -> Result<Box<ASTNode>, ASTError> {
        let t = self.lexer.next_token(true);
        match t.class {
            TokenClass::Macro(Macro::Include) => {
                let path = self.next_ast()?;
                return Ok(create_node!(ASTClass::MacroInclude(path)));
            }
            TokenClass::Macro(Macro::Undef) => {
                let id = self.next_ast()?;
                return Ok(create_node!(ASTClass::MacroUndef(id)));
            }
            TokenClass::Macro(Macro::Ifdef) => {
                let id = self.next_ast()?;
                return Ok(create_node!(ASTClass::MacroIfdef(id)));
            }
            TokenClass::Macro(Macro::Ifndef) => {
                let id = self.next_ast()?;
                return Ok(create_node!(ASTClass::MacroIfndef(id)));
            }
            TokenClass::Macro(Macro::Endif) => {
                return Ok(create_node!(ASTClass::MacroEndif));
            }
            TokenClass::Macro(Macro::Else) => {
                return Ok(create_node!(ASTClass::MacroElse));
            }
            TokenClass::Macro(Macro::Define) => {
                let id = self.next_ast()?;
                let val = self.get_string_or_newline_for_define().unwrap();
                return Ok(create_node!(ASTClass::MacroDefine(id, val)));
            }
            _ => {
                return Err(ASTError::UnExpectedToken(t, line!()));
            }
        }
    }

    fn get_string_or_newline_for_define(&mut self) -> Result<String, String> {
        let mut t_list: Vec<Token> = vec![];
        loop {
            let t = self.lexer.next_token(false);
            match t.class {
                TokenClass::Newline | TokenClass::EndOfProgram => {
                    match t_list.last() {
                        Some(ref t) => {
                            let str_vec = t_list.iter().map(|t| format!("{}", t)).collect::<Vec<String>>();
                            let result = str_vec.join("");
                            // セミコロンのトークンのfmt::Displayの実装は、"; "となっていて
                            // 後ろに空白を入れているが、
                            // 最後にセミコロンが来た場合のみ、
                            // 後ろの空白を削除して、最後の余分な空白を消している
                            if t.class == TokenClass::Symbol(Symbol::Semicolon) {
                                return Ok(result.trim_right().to_string());
                            }
                            return Ok(result);
                        }
                        None => {
                            return Ok("".to_string());
                        }
                    }
                }
                _ => {
                    t_list.push(t);
                }
            }
        }
    }

    fn generate_args_vec(&mut self) -> Result<Vec<Box<ASTNode>>, ASTError> {
        let left_paren = self.lexer.next_token(true);
        if let TokenClass::Symbol(Symbol::LeftParen) = left_paren.class {
        } else {
            return Err(ASTError::UnExpectedToken(left_paren, line!()));
        }

        let mut args = Vec::new();
        loop {
            let token = self.lexer.next_token(true);
            match token.class {
                TokenClass::Symbol(Symbol::RightParen) => {
                    return Ok(args);
                }
                TokenClass::Symbol(Symbol::Comma) => {}
                TokenClass::Identifire(id) => {
                    args.push(create_node!(ASTClass::Identifire(id)));
                }
                _ => {
                    return Err(ASTError::UnExpectedToken(token, line!()));
                }
            }
        }
    }

    fn create_expression(
        &mut self,
        node: Box<ASTNode>,
    ) -> Result<Box<ASTNode>, ASTError> {
        match self.lexer.check_next_token(true).class {
            TokenClass::Operator(_) => {
                let t = self.lexer.next_token(true);
                match t.class {
                    TokenClass::Operator(op) => {
                        let right = self.next_ast()?;
                        let op_node = create_node!(ASTClass::Operator(op));
                        let expr =
                            create_node!(ASTClass::Expression(node, op_node, right));
                        return self.create_expression(expr);
                    }
                    _ => {
                        return Err(ASTError::UnExpectedToken(t, line!()));
                    }
                }
            }
            _ => {
                return Ok(node);
            }
        }
    }
}
