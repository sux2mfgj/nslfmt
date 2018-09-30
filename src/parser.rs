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
        match self.lexer.check_next_token().unwrap().class {
            TokenClass::Newline => {
                self.lexer.next_token_nl();
                return Ok(create_node!(ASTClass::Newline));
            }
            _ => {
                return self.next_ast();
            }
        }
    }

    pub fn next_ast(&mut self) -> Result<Box<ASTNode>, ASTError> {
        let t = self.lexer.next_token();
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

                match self.lexer.check_next_token().unwrap().class {
                    TokenClass::Symbol(Symbol::Semicolon) => {
                        self.lexer.next_token();
                        return Ok(create_node!(ASTClass::Input(
                            id,
                            create_node!(ASTClass::Number("1".to_string()))
                        )));
                    }
                    TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                        let width = self.next_ast()?;
                        let _semicolon = self.lexer.next_token();
                        return Ok(create_node!(ASTClass::Input(id, width)));
                    }
                    _ => {
                        return Err(ASTError::UnExpectedToken(
                            self.lexer.next_token(),
                            line!(),
                        ));
                    }
                }
            }
            //TODO use macro. almost same with Input and InOut.
            TokenClass::Symbol(Symbol::Output) => {
                let id = self.next_ast()?;

                match self.lexer.check_next_token().unwrap().class {
                    TokenClass::Symbol(Symbol::Semicolon) => {
                        self.lexer.next_token();
                        return Ok(create_node!(ASTClass::Output(
                            id,
                            create_node!(ASTClass::Number("1".to_string()))
                        )));
                    }
                    TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                        let width = self.next_ast()?;
                        let _semicolon = self.lexer.next_token();
                        return Ok(create_node!(ASTClass::Output(id, width)));
                    }
                    _ => {
                        return Err(ASTError::UnExpectedToken(
                            self.lexer.next_token(),
                            line!(),
                        ));
                    }
                }
            }
            //TODO use macro
            TokenClass::Symbol(Symbol::InOut) => {
                let id = self.next_ast()?;

                match self.lexer.check_next_token().unwrap().class {
                    TokenClass::Symbol(Symbol::Semicolon) => {
                        self.lexer.next_token();
                        return Ok(create_node!(ASTClass::InOut(
                            id,
                            create_node!(ASTClass::Number("1".to_string()))
                        )));
                    }
                    TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                        let width = self.next_ast()?;
                        let _semicolon = self.lexer.next_token();
                        return Ok(create_node!(ASTClass::InOut(id, width)));
                    }
                    _ => {
                        return Err(ASTError::UnExpectedToken(
                            self.lexer.next_token(),
                            line!(),
                        ));
                    }
                }
            }
            TokenClass::Symbol(Symbol::FuncIn) => {
                let id = self.next_ast()?;
                let args = self.generate_args_vec()?;

                if let TokenClass::Symbol(Symbol::Colon) = self.lexer.next_token().class {
                    let out_token = self.lexer.next_token();
                    if let TokenClass::Identifire(s) = out_token.class {
                        let _semicolon = self.lexer.next_token();
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

                if let TokenClass::Symbol(Symbol::Colon) = self.lexer.next_token().class {
                    let out_token = self.lexer.next_token();
                    if let TokenClass::Identifire(s) = out_token.class {
                        let _semicolon = self.lexer.next_token();
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
                    match self.lexer.check_next_token().unwrap().class {
                        TokenClass::Symbol(Symbol::ClosingBrace) => {
                            self.lexer.next_token();
                            self.number_of_next -= 1;
                            return Ok(create_node!(ASTClass::Block(
                                content,
                                self.number_of_next
                            )));
                        }
                        TokenClass::EndOfProgram => {
                            return Err(ASTError::UnExpectedToken(
                                self.lexer.next_token(),
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
                let next_token = self.lexer.next_token();
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
                let next_token = self.lexer.next_token();
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
            TokenClass::Comment(line) => {
                return Ok(create_node!(ASTClass::Comment(line)));
            }
            _ => {
                panic!("unexptected token: {}", t);
            }
        }
    }

    fn generate_macro_astnode(&mut self) -> Result<Box<ASTNode>, ASTError> {
        let t = self.lexer.next_token();
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
            let t = self.lexer.next_token_nl();
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
        let left_paren = self.lexer.next_token();
        if let TokenClass::Symbol(Symbol::LeftParen) = left_paren.class {
        } else {
            return Err(ASTError::UnExpectedToken(left_paren, line!()));
        }

        let mut args = Vec::new();
        loop {
            let token = self.lexer.next_token();
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
        match self.lexer.check_next_token().unwrap().class {
            TokenClass::Operator(_) => {
                let t = self.lexer.next_token();
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

        let mut interfaces = vec![create_node!(ASTClass::Newline)];
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
        let mut b = BufReader::new(File::open("test_code/declare_03.nsl").unwrap());
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        let mut interfaces = Vec::new();
        interfaces.push(create_node!(ASTClass::Newline));
        interfaces.push(create_node!(ASTClass::Input(
            create_node!(ASTClass::Identifire("ok".to_string())),
            create_node!(ASTClass::Number("1".to_string()))
        )));
        interfaces.push(create_node!(ASTClass::Newline));
        interfaces.push(create_node!(ASTClass::Input(
            create_node!(ASTClass::Identifire("ggrks".to_string())),
            create_node!(ASTClass::Number("1".to_string()))
        )));
        interfaces.push(create_node!(ASTClass::Newline));
        interfaces.push(create_node!(ASTClass::Output(
            create_node!(ASTClass::Identifire("jk".to_string())),
            create_node!(ASTClass::Number("1".to_string()))
        )));
        interfaces.push(create_node!(ASTClass::Newline));
        interfaces.push(create_node!(ASTClass::Newline));

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
        interfaces.push(create_node!(ASTClass::Newline));
        interfaces.push(func2);
        interfaces.push(create_node!(ASTClass::Newline));

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
}
