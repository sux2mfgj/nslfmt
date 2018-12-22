use ast::*;
use lexer::*;
use token::*;
use std::panic;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    number_of_nest: usize,
}

#[macro_export]
macro_rules! create_node {
    ($n:expr) => {
        Box::new(ASTNode::new($n))
    };
}

macro_rules! not_implemented {
    () => {
        panic!("not implemented yet. at line {} in {}.", line!(), file!())
    };
}

macro_rules! unexpected_token {
    ($n:expr) => {
        panic!("unexpected_token {:?}", $n)
    };
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer: lexer,
            number_of_nest: 1,
        }
    }

    pub fn next_ast(&mut self) -> Box<ASTNode> {
        let token = self.lexer.next();
        match token.class {
            TokenClass::Symbol(Symbol::Declare) => self.declare_ast(),
            _ => {
                unexpected_token!(token);
            }
        }
    }

    fn declare_ast(&mut self) -> Box<ASTNode> {
        // <identifire>
        let id_node = self.generate_id_node();
        self.check_opening_brace();
        let mut contents_in_block = vec![];
        loop {
            {
                let next = &self.lexer.peek();
                if let TokenClass::Symbol(Symbol::ClosingBrace) = next.class
                {
                    return create_node!(ASTClass::Declare(
                            id_node,
                            create_node!(ASTClass::Block(contents_in_block))
                            ));
                }
            }
            let declare_block = self.declare_block_part_ast();
            contents_in_block.push(declare_block);
        }
    }

    fn declare_block_part_ast(&mut self) -> Box<ASTNode> {
        let t = self.lexer.next();
        match t.class {
            TokenClass::Newline => {
                return create_node!(ASTClass::Newline);
            }
            TokenClass::Symbol(Symbol::Input) => {
                let id_node = self.generate_id_node();
                return match self.lexer.next().class {
                    TokenClass::Symbol(Symbol::Semicolon) => {
                        create_node!(ASTClass::Input(
                                id_node,
                                None
                                ))
                    }
                    TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                        let expr = self.expression_ast();
                        self.check_right_square_bracket();
                        self.check_semicolon();
                        create_node!(ASTClass::Input(
                                id_node,
                                Some(expr)))
                    }
                    _ => {
                        not_implemented!();
                    }
                }
            }
            TokenClass::Symbol(Symbol::FuncIn) => {
                let id_token = self.lexer.next();
                if let TokenClass::Identifire(id_str) = id_token.class {
                    self.check_left_paren();
                    let args_vec = self.generate_args_vec();
                    let return_port = self.generate_func_return();
                    /*
                    return create_node!(ASTClass::FuncIn(
                        create_node!(ASTClass::Identifire(id_str.to_string())),
                        args_vec,
                        return_port,
                    ));
                    */
                    not_implemented!();
                } else {
                    unexpected_token!(id_token);
                }

            }
            _ => {
                not_implemented!();
            }
        }
    }

    fn to_node(& self, t: Token) -> Box<ASTNode>
    {
        return match t.class
        {
            TokenClass::Number(num) => {
                create_node!(ASTClass::Number(num))
            }
            TokenClass::Identifire(id) => {
                create_node!(ASTClass::Identifire(id))
            }
            _ => {
                unexpected_token!(t);
            }
        }

    }

    fn expression_ast(&mut self) -> (Box<ASTNode>) {
        let left = self.lexer.next();
        if let TokenClass::Operator(op) = self.lexer.peek().class
        {
            self.lexer.next();
            return create_node!(ASTClass::Expression(
                    self.to_node(left),
                    create_node!(ASTClass::Operator(op)),
                    self.expression_ast()
                    ));

        }
        self.to_node(left)
    }

    fn generate_id_node(&mut self) -> Box<ASTNode> {
        let id_token = self.lexer.next();
        if let TokenClass::Identifire(id_str) = id_token.class {
            return create_node!(ASTClass::Identifire(id_str));
        } else {
            unexpected_token!(id_token)
        }
    }

    fn generate_args_vec(&mut self) -> Vec<Box<ASTNode>> {
        let mut args = vec![];
        loop {
            let token = self.lexer.next();
            match token.class {
                TokenClass::Symbol(Symbol::RightParen) => {
                    break;
                }
                TokenClass::Symbol(Symbol::Comma) => {
                    continue;
                }
                TokenClass::Identifire(id_str) => {
                    args.push(create_node!(ASTClass::Identifire(id_str)));
                }
                TokenClass::Number(num) => {
                    args.push(create_node!(ASTClass::Number(num)));
                }
                _ => {
                    unexpected_token!(token);
                }
            }
        }

        args
    }

    fn generate_func_return(&mut self) -> Option<Box<ASTNode>> {
        let colon_token = self.lexer.next();

        return if TokenClass::Symbol(Symbol::Colon) == colon_token.class {
            let port_id = self.lexer.next();

            if let TokenClass::Identifire(id_str) = port_id.class {
                self.check_semicolon();
                Some(create_node!(ASTClass::Identifire(id_str)))
            } else {
                unexpected_token!(port_id);
            }
        } else {
            self.check_semicolon();
            None
        }
    }

    fn check_opening_brace(&mut self) {
        let token = self.lexer.next();
        if TokenClass::Symbol(Symbol::OpeningBrace) != token.class {
            unexpected_token!(token);
        }
    }

    fn check_right_square_bracket(&mut self) {
        let token = self.lexer.next();
        if TokenClass::Symbol(Symbol::RightSquareBracket) != token.class {
            unexpected_token!(token);
        }
    }

    fn check_semicolon(&mut self) {
        let token = self.lexer.next();
        if TokenClass::Symbol(Symbol::Semicolon) != token.class {
            unexpected_token!(token);
        }
    }

    fn check_left_paren(&mut self) {
        let token = self.lexer.next();
        if TokenClass::Symbol(Symbol::LeftParen) != token.class {
            unexpected_token!(token);
        }
    }
}
