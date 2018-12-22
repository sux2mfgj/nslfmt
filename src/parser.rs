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
            TokenClass::Symbol(Symbol::Sharp) => self.macro_ast(),
            TokenClass::Symbol(Symbol::Declare) => self.declare_ast(),

            TokenClass::EndOfProgram => create_node!(ASTClass::EndOfProgram),
            _ => {
                unexpected_token!(token);
            }
        }
    }

    fn macro_ast(&mut self) -> Box<ASTNode> {
        let macro_kind_token = self.lexer.next();
        match macro_kind_token.class {
            TokenClass::Macro(Macro::Include) => {
                create_node!(ASTClass::MacroInclude(self.generate_path_node()))
            }
            TokenClass::Macro(Macro::Undef) => {
                let id = self.generate_id_node();
                create_node!(ASTClass::MacroUndef(id))
            }
            TokenClass::Macro(Macro::Ifdef) => {
                let id = self.generate_id_node();
                create_node!(ASTClass::MacroIfdef(id))
            }
            TokenClass::Macro(Macro::Ifndef) => {
                let id = self.generate_id_node();
                create_node!(ASTClass::MacroIfndef(id))
            }
            TokenClass::Macro(Macro::Endif) => {
                create_node!(ASTClass::MacroEndif)
            }
            TokenClass::Symbol(Symbol::Else) => {
                create_node!(ASTClass::MacroElse)
            }
            TokenClass::Macro(Macro::Define) => {
                let id = self.generate_id_node();
                let value = self.generate_string_until_nl();
                create_node!(ASTClass::MacroDefine(id, value))
            }
            _ => {
                unexpected_token!(macro_kind_token);
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

    fn get_id_and_width(&mut self) -> (Box<ASTNode>, Option<Box<ASTNode>>)
    {
        let id_node = self.generate_id_node();
        match self.lexer.next().class {
            TokenClass::Symbol(Symbol::Semicolon) => {
                (id_node, None)
            }
            TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                let expr = self.expression_ast();
                self.check_right_square_bracket();
                self.check_semicolon();
                (id_node, Some(expr))
            }
            _ => {
                not_implemented!();
            }
        }
    }

    fn declare_block_part_ast(&mut self) -> Box<ASTNode> {
        let t = self.lexer.next();
        return match t.class {
            TokenClass::Symbol(Symbol::Input) => {
                let (id_node, width) = self.get_id_and_width();
                create_node!(ASTClass::Input(id_node, width))
            }
            TokenClass::Symbol(Symbol::Output) => {
                let (id_node, width) = self.get_id_and_width();
                create_node!(ASTClass::Output(id_node, width))
            }
            TokenClass::Symbol(Symbol::InOut) => {
                let (id_node, width) = self.get_id_and_width();
                create_node!(ASTClass::InOut(id_node, width))
            }
            TokenClass::Symbol(Symbol::FuncIn) => {
                let id_token = self.lexer.next();
                if let TokenClass::Identifire(id_str) = id_token.class {
                    self.check_left_paren();
                    let args_vec = self.generate_args_vec();
                    let return_port = self.generate_func_return();
                    create_node!(ASTClass::FuncIn(
                        create_node!(ASTClass::Identifire(id_str.to_string())),
                        args_vec,
                        return_port,
                    ))
                } else {
                    unexpected_token!(id_token);
                }
            }
            TokenClass::Symbol(Symbol::FuncOut) => {
                let t = self.lexer.next();
                if let TokenClass::Identifire(id) = t.class {
                    self.check_left_paren();
                    let args_vec = self.generate_args_vec();
                    let return_port = self.generate_func_return();
                    create_node!(ASTClass::FuncOut(
                        create_node!(ASTClass::Identifire(id.to_string())),
                        args_vec,
                        return_port,
                    ))
                }
                else
                {
                    unexpected_token!(t);
                }
            }
            _ => {
                unexpected_token!(t);
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
        let colon_token = self.lexer.peek();

        return if TokenClass::Symbol(Symbol::Colon) == colon_token.class {
            self.lexer.next();
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

    fn generate_string_until_nl(&mut self) -> Option<String> {
        let mut t_list: Vec<Token> = vec![];
        loop {
            let t = self.lexer.next();
            match t.class {
                TokenClass::Newline | TokenClass::EndOfProgram => {
                    match t_list.last() {
                        Some(ref t) => {
                            let str_vec = t_list
                                .iter()
                                .map(|t| format!("{}", t))
                                .collect::<Vec<String>>();
                            let result = str_vec.join("");
                            // セミコロンのトークンのfmt::Displayの実装は、"; "となっていて
                            // 後ろに空白を入れているが、
                            // 最後にセミコロンが来た場合のみ、
                            // 後ろの空白を削除して、最後の余分な空白を消している
                            if t.class == TokenClass::Symbol(Symbol::Semicolon) {
                                return Some(result.trim_right().to_string());
                            }
                            return Some(result);
                        }
                        None => return None,
                    }
                }
                _ => {
                    t_list.push(t);
                }
            }
        }
    }

    fn generate_path_node(&mut self) -> Box<ASTNode> {
        let path_token = self.lexer.next();
        if let TokenClass::String(id_str) = path_token.class {
            return create_node!(ASTClass::String(id_str));
        }
        unexpected_token!(path_token);
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
