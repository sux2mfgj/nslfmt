use ast::*;
use lexer::*;
use token::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTError {
    UnExpectedToken(Token, u32),
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    number_of_nest: usize,
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
            number_of_nest: 1,
        }
    }

    pub fn next_ast_top(&mut self) -> Result<Box<ASTNode>, ASTError> {
        let t = self.lexer.next_token(false);
        match t.class {
            TokenClass::Symbol(Symbol::Sharp) => self.generate_macro_astnode(),
            TokenClass::Symbol(Symbol::Declare) => self.declare_ast(),
            TokenClass::Symbol(Symbol::Module) => self.module_ast(),
            TokenClass::CPPStyleComment(list) => {
                return Ok(create_node!(ASTClass::CPPStyleComment(list)));
            }
            TokenClass::Newline => Ok(create_node!(ASTClass::Newline)),
            TokenClass::EndOfProgram => Ok(create_node!(ASTClass::EndOfProgram)),
            _ => {
                panic!("unexptected token {:?}", t);
            }
        }
    }

    /*
     * declare <id>
     * {
     *      <interfaces>
     * }
     */
    pub fn declare_ast(&mut self) -> Result<Box<ASTNode>, ASTError> {
        // <id>
        let id_token = self.lexer.next_token(true);
        let brace_token = self.lexer.next_token(false);

        if let (
            TokenClass::Identifire(id_str),
            TokenClass::Symbol(Symbol::OpeningBrace),
        ) = (id_token.class, brace_token.class)
        {
            let mut content = Vec::new();
            loop {
                let next_t = self.lexer.check_next_token(false);
                match next_t.class {
                    TokenClass::Symbol(Symbol::ClosingBrace) => {
                        self.lexer.next_token(true);
                        return Ok(create_node!(ASTClass::Declare(
                            create_node!(ASTClass::Identifire(id_str)),
                            create_node!(ASTClass::Block(content, 1))
                        )));
                    }
                    TokenClass::EndOfProgram => {
                        panic!("unexptected EOP {:?}", next_t);
                    }
                    _ => {
                        let t = self.declare_block_ast()?;
                        content.push(t);
                    }
                }
            }
        } else {
            panic!("test");
        }
    }

    pub fn module_ast(&mut self) -> Result<Box<ASTNode>, ASTError> {
        // <id>
        let id_token = self.lexer.next_token(true);
        let brace_token = self.lexer.next_token(false);
        if let (
            TokenClass::Identifire(id_str),
            TokenClass::Symbol(Symbol::OpeningBrace),
        ) = (id_token.class, brace_token.class)
        {
            let mut content = Vec::new();
            loop {
                let next_t = self.lexer.check_next_token(false);
                match next_t.class {
                    TokenClass::Symbol(Symbol::ClosingBrace) => {
                        self.lexer.next_token(true);
                        return Ok(create_node!(ASTClass::Module(
                            create_node!(ASTClass::Identifire(id_str)),
                            create_node!(ASTClass::Block(content, 1))
                        )));
                    }
                    TokenClass::EndOfProgram => {
                        panic!("unexptected EOP {:?}", next_t);
                    }
                    _ => {
                        let t = self.module_block_ast()?;
                        content.push(t);
                    }
                }
            }
        } else {
            panic!("unexptected token");
        }
    }

    fn module_block_ast(&mut self) -> Result<Box<ASTNode>, ASTError> {
        let t = self.lexer.next_token(false);
        match t.class {
            TokenClass::Symbol(Symbol::Wire) => {
                let mut wire_list = vec![];
                while let Some(def) = self.wire_defines() {
                    wire_list.push(def);
                }

                return Ok(create_node!(ASTClass::Wire(wire_list)));
            }
            TokenClass::Symbol(Symbol::Reg) => {
                let mut reg_list = vec![];
                while let Some(def) = self.reg_defines() {
                    reg_list.push(def);
                }
                return Ok(create_node!(ASTClass::Reg(reg_list)));
            }
            TokenClass::Symbol(Symbol::FuncSelf) => {
                let id = self.get_identifire()?;
                let mut n_t = self.lexer.check_next_token(true);

                let mut args: Option<Vec<Box<ASTNode>>> = None;
                let mut ret: Option<Box<ASTNode>> = None;

                if n_t.class == TokenClass::Symbol(Symbol::LeftParen) {
                    args = Some(self.generate_args_vec()?);
                    n_t = self.lexer.check_next_token(true);
                }

                if n_t.class == TokenClass::Symbol(Symbol::Colon) {
                    // pass colon
                    let _t = self.lexer.next_token(true);
                    ret = Some(self.get_identifire()?);
                }

                // pass semicolon
                let _t = self.lexer.next_token(true);

                return Ok(create_node!(ASTClass::FuncSelf(id, args, ret)));
            }
            _ => {
                panic!("unexptected token {:?}", t);
            }
        }
    }

    pub fn declare_block_ast(&mut self) -> Result<Box<ASTNode>, ASTError> {
        let t = self.lexer.next_token(false);
        match t.class {
            TokenClass::Symbol(Symbol::Input) => {
                if let TokenClass::Identifire(id_str) = self.lexer.next_token(true).class
                {
                    match self.lexer.check_next_token(true).class {
                        TokenClass::Symbol(Symbol::Semicolon) => {
                            self.lexer.next_token(true);
                            Ok(create_node!(ASTClass::Input(
                                create_node!(ASTClass::Identifire(id_str)),
                                None
                            )))
                        }
                        TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                            let width = self.width_expression_ast()?;
                            let _semicolon = self.lexer.next_token(true);
                            Ok(create_node!(ASTClass::Input(
                                create_node!(ASTClass::Identifire(id_str)),
                                Some(width)
                            )))
                        }
                        _ => {
                            panic!("unexptected token {:?}", t);
                        }
                    }
                } else {
                    panic!("unexptected token");
                }
            }
            TokenClass::Symbol(Symbol::Output) => {
                if let TokenClass::Identifire(id_str) = self.lexer.next_token(true).class
                {
                    match self.lexer.check_next_token(true).class {
                        TokenClass::Symbol(Symbol::Semicolon) => {
                            self.lexer.next_token(true);
                            Ok(create_node!(ASTClass::Output(
                                create_node!(ASTClass::Identifire(id_str)),
                                None
                            )))
                        }
                        TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                            let width = self.width_expression_ast()?;
                            let _semicolon = self.lexer.next_token(true);
                            Ok(create_node!(ASTClass::Output(
                                create_node!(ASTClass::Identifire(id_str)),
                                Some(width)
                            )))
                        }
                        _ => {
                            panic!("unexptected token {:?}", t);
                        }
                    }
                } else {
                    panic!("unexptected token");
                }
            }
            TokenClass::Symbol(Symbol::InOut) => {
                if let TokenClass::Identifire(id_str) = self.lexer.next_token(true).class
                {
                    match self.lexer.check_next_token(true).class {
                        TokenClass::Symbol(Symbol::Semicolon) => {
                            self.lexer.next_token(true);
                            Ok(create_node!(ASTClass::InOut(
                                create_node!(ASTClass::Identifire(id_str)),
                                None
                            )))
                        }
                        TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                            let width = self.width_expression_ast()?;
                            let _semicolon = self.lexer.next_token(true);
                            Ok(create_node!(ASTClass::InOut(
                                create_node!(ASTClass::Identifire(id_str)),
                                Some(width)
                            )))
                        }
                        _ => {
                            panic!("unexptected token {:?}", t);
                        }
                    }
                } else {
                    panic!("unexptected token");
                }
            }
            TokenClass::Symbol(Symbol::FuncIn) => {
                if let TokenClass::Identifire(id_str) = self.lexer.next_token(true).class
                {
                    let args = self.generate_args_vec()?;
                    if let TokenClass::Symbol(Symbol::Colon) =
                        self.lexer.next_token(true).class
                    {
                        let out_token = self.lexer.next_token(true);
                        if let TokenClass::Identifire(s) = out_token.class {
                            let _semicolon = self.lexer.next_token(true);
                            let return_node = create_node!(ASTClass::Identifire(s));
                            Ok(create_node!(ASTClass::FuncIn(
                                create_node!(ASTClass::Identifire(id_str)),
                                args,
                                Some(return_node)
                            )))
                        } else {
                            panic!("unexptected token {:?}", out_token);
                        }
                    } else {
                        Ok(create_node!(ASTClass::FuncIn(
                            create_node!(ASTClass::Identifire(id_str)),
                            args,
                            None,
                        )))
                    }
                } else {
                    panic!("unexptected token");
                }
            }
            TokenClass::Symbol(Symbol::FuncOut) => {
                if let TokenClass::Identifire(id_str) = self.lexer.next_token(true).class
                {
                    let args = self.generate_args_vec()?;
                    if let TokenClass::Symbol(Symbol::Colon) =
                        self.lexer.next_token(true).class
                    {
                        let out_token = self.lexer.next_token(true);
                        if let TokenClass::Identifire(s) = out_token.class {
                            let _semicolon = self.lexer.next_token(true);
                            let return_node = create_node!(ASTClass::Identifire(s));
                            Ok(create_node!(ASTClass::FuncOut(
                                create_node!(ASTClass::Identifire(id_str)),
                                args,
                                Some(return_node)
                            )))
                        } else {
                            panic!("unexptected token {:?}", out_token);
                        }
                    } else {
                        Ok(create_node!(ASTClass::FuncOut(
                            create_node!(ASTClass::Identifire(id_str)),
                            args,
                            None,
                        )))
                    }
                } else {
                    panic!("unexptected token");
                }
            }
            TokenClass::Newline => Ok(create_node!(ASTClass::Newline)),
            _ => {
                panic!("unexptected token {:?}", t);
            }
        }
    }

    pub fn width_expression_ast(&mut self) -> Result<Box<ASTNode>, ASTError> {
        let t = self.lexer.next_token(true);
        match t.class {
            TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                let left = self.next_ast()?;
                let expr = self.create_expression(left)?;
                let next_token = self.lexer.next_token(true);
                match next_token.class {
                    TokenClass::Symbol(Symbol::RightSquareBracket) => Ok(expr),
                    _ => {
                        panic!("unexptected token {:?}", next_token);
                    }
                }
            }
            _ => {
                panic!("unexptected token {:?}", t);
            }
        }
    }

    pub fn next_ast(&mut self) -> Result<Box<ASTNode>, ASTError> {
        let t = self.lexer.next_token(true);
        match t.class {
            TokenClass::Identifire(s) => Ok(create_node!(ASTClass::Identifire(s))),
            TokenClass::Number(n) => Ok(create_node!(ASTClass::Number(n))),
            _ => {
                panic!("unexptected token {:?}", t);
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
                        self.create_expression(expr)
                    }
                    _ => {
                        panic!("unexptected token {:?}", t);
                    }
                }
            }
            _ => Ok(node),
        }
    }

    fn generate_args_vec(&mut self) -> Result<Vec<Box<ASTNode>>, ASTError> {
        let left_paren = self.lexer.next_token(true);
        if let TokenClass::Symbol(Symbol::LeftParen) = left_paren.class {
        } else {
            panic!("unexptected token {:?}", left_paren);
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
                    panic!("unexptected token {:?}", token);
                }
            }
        }
    }

    fn get_path(&mut self) -> Result<Box<ASTNode>, ASTError> {
        let t = self.lexer.next_token(true);
        if let TokenClass::String(id_str) = t.class {
            Ok(create_node!(ASTClass::String(id_str)))
        } else {
            Err(ASTError::UnExpectedToken(t, line!()))
        }
    }

    fn get_identifire(&mut self) -> Result<Box<ASTNode>, ASTError> {
        let t = self.lexer.next_token(true);
        if let TokenClass::Identifire(id_str) = t.class {
            Ok(create_node!(ASTClass::Identifire(id_str)))
        } else {
            Err(ASTError::UnExpectedToken(t, line!()))
        }
    }

    fn get_string_or_newline_for_define(&mut self) -> Option<String> {
        let mut t_list: Vec<Token> = vec![];
        loop {
            let t = self.lexer.next_token(false);
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

    fn generate_macro_astnode(&mut self) -> Result<Box<ASTNode>, ASTError> {
        let t = self.lexer.next_token(true);
        match t.class {
            TokenClass::Macro(Macro::Include) => {
                let path = self.get_path()?;
                return Ok(create_node!(ASTClass::MacroInclude(path)));
            }
            TokenClass::Macro(Macro::Undef) => {
                let id = self.get_identifire()?;
                return Ok(create_node!(ASTClass::MacroUndef(id)));
            }
            TokenClass::Macro(Macro::Ifdef) => {
                let id = self.get_identifire()?;
                return Ok(create_node!(ASTClass::MacroIfdef(id)));
            }
            TokenClass::Macro(Macro::Ifndef) => {
                let id = self.get_identifire()?;
                return Ok(create_node!(ASTClass::MacroIfndef(id)));
            }
            TokenClass::Macro(Macro::Endif) => {
                return Ok(create_node!(ASTClass::MacroEndif));
            }
            TokenClass::Macro(Macro::Else) => {
                return Ok(create_node!(ASTClass::MacroElse));
            }
            TokenClass::Macro(Macro::Define) => {
                let id = self.get_identifire()?;
                let val = self.get_string_or_newline_for_define();
                return Ok(create_node!(ASTClass::MacroDefine(id, val)));
            }
            _ => {
                return Err(ASTError::UnExpectedToken(t, line!()));
            }
        }
    }

    fn wire_defines(&mut self) -> Option<(Box<ASTNode>, Option<Box<ASTNode>>)> {
        let t = self.lexer.check_next_token(true);
        match t.class {
            TokenClass::Symbol(Symbol::Semicolon) => {
                // pass a semicolon
                self.lexer.next_token(true);
                return None;
            }
            TokenClass::Symbol(Symbol::Comma) => {
                // pass a comma
                self.lexer.next_token(true);
                return self.wire_defines();
            }
            TokenClass::Identifire(_) => {
                let id = self.next_ast().unwrap();
                let next_t = self.lexer.check_next_token(true);
                match next_t.class {
                    TokenClass::Symbol(Symbol::Semicolon) => {
                        return Some((id, None));
                    }
                    TokenClass::Symbol(Symbol::Comma) => {
                        // pass a comma
                        self.lexer.next_token(true);
                        return Some((id, None));
                    }
                    TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                        let width = self.width_expression_ast().unwrap();
                        return Some((id, Some(width)));
                    }
                    _ => {
                        panic!("{:?}", next_t);
                    }
                }
            }
            _ => {
                panic!("unexptected token: {:?}", t);
            }
        }
    }

    fn reg_defines(
        &mut self,
    ) -> Option<(Box<ASTNode>, Option<Box<ASTNode>>, Option<Box<ASTNode>>)> {
        let t = self.lexer.check_next_token(true);
        match t.class {
            TokenClass::Symbol(Symbol::Semicolon) => {
                // pass a semicolon
                self.lexer.next_token(true);
                return None;
            }
            TokenClass::Symbol(Symbol::Comma) => {
                // pass a comma
                self.lexer.next_token(true);
                return self.reg_defines();
            }
            TokenClass::Identifire(_) => {
                let id = self.next_ast().unwrap();
                let next_t = self.lexer.check_next_token(true);
                match next_t.class {
                    TokenClass::Symbol(Symbol::Semicolon) => {
                        return Some((id, None, None));
                    }
                    TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                        let width_expr = self.width_expression_ast().unwrap();
                        let nn_t = self.lexer.check_next_token(true);

                        let mut init: Option<Box<ASTNode>> = None;
                        if nn_t.class == TokenClass::Symbol(Symbol::Equal) {
                            let _eq = self.lexer.next_token(true);
                            init = Some(self.next_ast().unwrap());
                        }
                        return Some((id, Some(width_expr), init));
                    }
                    TokenClass::Symbol(Symbol::Comma) => {
                        // pass a comma
                        self.lexer.next_token(true);
                        return Some((id, None, None));
                    }
                    TokenClass::Symbol(Symbol::Equal) => {
                        let _eq = self.lexer.next_token(true);
                        let init = Some(self.next_ast().unwrap());
                        return Some((id, None, init));
                    }
                    _ => {
                        panic!("??? {:?}", next_t);
                    }
                }
            }
            _ => {
                panic!("?? {:?}", t);
            }
        }
    }
}
