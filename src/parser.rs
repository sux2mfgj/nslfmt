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
            TokenClass::Symbol(Symbol::Module) => {
                let (id, block) = self.module_ast();
                return Ok(create_node!(ASTClass::Module(id, block)));
            }
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
        let brace_token = self.lexer.next_token(true);

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
                            create_node!(ASTClass::Block(content))
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

    //                              id           , block
    pub fn module_ast(&mut self) -> (Box<ASTNode>, Box<ASTNode>) {
        // <id>
        let id_token = self.lexer.next_token(true);

        if let TokenClass::Identifire(id_str) = id_token.class {
            let block = self.module_block();
            let id = create_node!(ASTClass::Identifire(id_str));
            return (id, block);
        } else {
            panic!("unexptected token {:?}", id_token);
        }
    }

    // TODO
    // consider a one line expression which doesn't have parenthesis.
    // like
    // any
    // {
    //      test:
    //          a := 4'b0010;
    // }
    fn module_block(&mut self) -> Box<ASTNode> {
        let brace_token = self.lexer.next_token(true);
        if TokenClass::Symbol(Symbol::OpeningBrace) == brace_token.class {
            let mut content = Vec::new();
            loop {
                let next_t = self.lexer.check_next_token(true);
                match next_t.class {
                    TokenClass::Symbol(Symbol::ClosingBrace) => {
                        self.lexer.next_token(true);
                        return create_node!(ASTClass::Block(content));
                    }
                    TokenClass::EndOfProgram => {
                        panic!("unexptected EOP {:?}", next_t);
                    }
                    _ => {
                        if let Some(t) = self.module_component_declares() {
                            content.push(t);
                        } else if let Some(t) = self.module_behavioral_description() {
                            content.push(t);
                        } else {
                            panic!("unexptected token {:?}", next_t);
                        }
                    }
                }
            }
        } else {
            panic!("unexptected token {:?}", brace_token);
        }
    }

    fn module_component_declares(&mut self) -> Option<Box<ASTNode>> {
        let t = self.lexer.check_next_token(true);
        match t.class {
            TokenClass::Symbol(Symbol::Wire) => {
                let _wire = self.lexer.next_token(true);
                let mut wire_list = vec![];
                while let Some(def) = self.wire_defines() {
                    wire_list.push(def);
                }

                return Some(create_node!(ASTClass::Wire(wire_list)));
            }
            TokenClass::Symbol(Symbol::Reg) => {
                let _reg = self.lexer.next_token(true);
                let mut reg_list = vec![];
                while let Some(def) = self.reg_defines() {
                    reg_list.push(def);
                }
                return Some(create_node!(ASTClass::Reg(reg_list)));
            }
            TokenClass::Symbol(Symbol::FuncSelf) => {
                let _func_self = self.lexer.next_token(true);
                let id = self.get_identifire().unwrap();
                let mut n_t = self.lexer.check_next_token(true);

                let mut args = vec![];
                let mut ret: Option<Box<ASTNode>> = None;

                if n_t.class == TokenClass::Symbol(Symbol::LeftParen) {
                    args = self.generate_args_vec(false);
                    n_t = self.lexer.check_next_token(true);
                }

                if n_t.class == TokenClass::Symbol(Symbol::Colon) {
                    // pass colon
                    let _t = self.lexer.next_token(true);
                    ret = Some(self.get_identifire().unwrap());
                }

                // pass semicolon
                let _t = self.lexer.next_token(true);

                return Some(create_node!(ASTClass::FuncSelf(id, args, ret)));
            }
            TokenClass::Symbol(Symbol::ProcName) => {
                let _proc_name = self.lexer.next_token(true);
                if let TokenClass::Identifire(id_str) = self.lexer.next_token(true).class
                {
                    let arg = self.generate_args_vec(false);
                    // pass semicolon
                    let _t = self.lexer.next_token(true);

                    Some(create_node!(ASTClass::ProcName(
                        create_node!(ASTClass::Identifire(id_str)),
                        arg
                    )))
                } else {
                    panic!("unexptected token")
                }
            }
            TokenClass::Symbol(Symbol::StateName) => {
                let _state_name = self.lexer.next_token(true);
                let mut states = vec![];
                loop {
                    let t = self.lexer.next_token(true);
                    match t.class {
                        TokenClass::Identifire(id) => {
                            states.push(id);
                        }
                        TokenClass::Symbol(Symbol::Comma) => {}
                        TokenClass::Symbol(Symbol::Semicolon) => {
                            return Some(create_node!(ASTClass::StateName(states)));
                        }
                        _ => {
                            panic!("unexptected token");
                        }
                    }
                }
            }
            TokenClass::Symbol(Symbol::Mem) => {
                let _mem = self.lexer.next_token(true);
                let mut id_ast: Box<ASTNode>;
                if let TokenClass::Identifire(id_str) = self.lexer.next_token(true).class
                {
                    id_ast = create_node!(ASTClass::Identifire(id_str));
                } else {
                    panic!("unexptected token");
                }

                let mut width1: Box<ASTNode>;
                if let TokenClass::Symbol(Symbol::LeftSquareBracket) =
                    self.lexer.check_next_token(true).class
                {
                    width1 = self.width_expression_ast().unwrap();
                } else {
                    panic!("unexptected token");
                }

                let mut width2: Option<Box<ASTNode>> = None;
                if let TokenClass::Symbol(Symbol::LeftSquareBracket) =
                    self.lexer.check_next_token(true).class
                {
                    width2 = self.width_expression_ast();
                }

                let mut init: Option<Vec<Box<ASTNode>>> = None;
                if let TokenClass::Symbol(Symbol::Equal) =
                    self.lexer.check_next_token(true).class
                {
                    //pass equal
                    self.lexer.next_token(true);
                    init = Some(self.generate_mem_init_vec());
                }

                let _t = self.lexer.next_token(true);
                return Some(create_node!(ASTClass::Mem(id_ast, width1, width2, init,)));
            }
            TokenClass::Identifire(id_str) => {
                let _id = self.lexer.next_token(true);
                let t = self.lexer.next_token(true);
                match t.class {
                    TokenClass::Symbol(Symbol::Equal) => {
                        let left = self.next_ast().unwrap();
                        let expr = self.create_expression(left).unwrap();
                        let _semicolon = self.lexer.next_token(true);
                        return Some(create_node!(ASTClass::Assign(
                            create_node!(ASTClass::Identifire(id_str)),
                            expr
                        )));
                    }
                    TokenClass::Symbol(Symbol::RegAssign) => {
                        let left = self.next_ast().unwrap();
                        let expr = self.create_expression(left).unwrap();
                        let _semicolon = self.lexer.next_token(true);
                        return Some(create_node!(ASTClass::Assign(
                            create_node!(ASTClass::Identifire(id_str)),
                            expr
                        )));
                    }
                    // function call
                    TokenClass::Symbol(Symbol::LeftParen) => {
                        let args = self.generate_args_vec(true);
                        // pass semicolon
                        let _t = self.lexer.next_token(true);
                        return Some(
                            create_node!(ASTClass::FuncCall(
                                    create_node!(ASTClass::Identifire(id_str)),
                                    args,
                                    ))
                            );
                    }
                    _ => {
                        panic!("unexptected token {:?}", t);
                    }
                }
            }
            _ => {
                return None;
            }
        }
    }

    fn module_behavioral_description(&mut self) -> Option<Box<ASTNode>> {
        let t = self.lexer.next_token(true);
        match t.class {
            TokenClass::Symbol(Symbol::Func) => {
                let (id, block) = self.module_ast();
                return Some(create_node!(ASTClass::Func(id, block)));
            }
            TokenClass::Symbol(Symbol::Return) => {
                let left = self.next_ast()?;
                let expr = self.create_expression(left).unwrap();
                let _semicolon = self.lexer.next_token(true);
                return Some(create_node!(ASTClass::Return(expr)));
            }
            TokenClass::Symbol(Symbol::Any) => {
                if let TokenClass::Symbol(Symbol::OpeningBrace) =
                    self.lexer.next_token(true).class
                {
                    let mut any_componens = vec![];
                    loop {
                        if let TokenClass::Symbol(Symbol::ClosingBrace) =
                            self.lexer.check_next_token(true).class
                        {
                            return Some(create_node!(ASTClass::Any(any_componens)));
                        }
                        let left = self.next_ast()?;
                        let expr = self.create_expression(left).unwrap();

                        if let TokenClass::Symbol(Symbol::Colon) =
                            self.lexer.next_token(true).class
                        {
                            let block = self.module_block();
                            any_componens.push((expr, block));
                        }
                    }
                }
                panic!("unexptected token");
            }
            _ => {
                return None;
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
                            let width = self.width_expression_ast();
                            let _semicolon = self.lexer.next_token(true);
                            Ok(create_node!(ASTClass::Input(
                                create_node!(ASTClass::Identifire(id_str)),
                                width
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
                            let width = self.width_expression_ast();
                            let _semicolon = self.lexer.next_token(true);
                            Ok(create_node!(ASTClass::Output(
                                create_node!(ASTClass::Identifire(id_str)),
                                width
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
                            let width = self.width_expression_ast();
                            let _semicolon = self.lexer.next_token(true);
                            Ok(create_node!(ASTClass::InOut(
                                create_node!(ASTClass::Identifire(id_str)),
                                width
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
                    let args = self.generate_args_vec(false);
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
                    let args = self.generate_args_vec(false);
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
            TokenClass::Identifire(_) => {
                let mut tokens = vec![t];
                while let Some(tt) = self.get_token_for_macro_in_declare() {
                    tokens.push(tt);
                }
                return Ok(create_node!(ASTClass::Macro_SubModule(tokens)));
            }
            _ => {
                panic!("unexptected token {:?}", t);
            }
        }
    }

    //     fn get_token_for_macro_in_module(&mut self) -> Option<Token> {
    //         let t = self.lexer.check_next_token(true);
    //         match t.class {
    //
    //         }
    //     }

    fn get_token_for_macro_in_declare(&mut self) -> Option<Token> {
        let t = self.lexer.check_next_token(true);
        match t.class {
            TokenClass::Symbol(Symbol::Input) => None,
            TokenClass::Symbol(Symbol::Output) => None,
            TokenClass::Symbol(Symbol::InOut) => None,
            TokenClass::Symbol(Symbol::FuncIn) => None,
            TokenClass::Symbol(Symbol::FuncOut) => None,
            TokenClass::Symbol(Symbol::Semicolon) => None,
            TokenClass::Symbol(Symbol::ClosingBrace) => None,
            _ => Some(self.lexer.next_token(true)),
        }
    }

    pub fn width_expression_ast(&mut self) -> Option<Box<ASTNode>> {
        let t = self.lexer.next_token(true);
        match t.class {
            TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                let left = self.next_ast()?;
                let expr = self.create_expression(left);
                let next_token = self.lexer.next_token(true);
                match next_token.class {
                    TokenClass::Symbol(Symbol::RightSquareBracket) => expr,
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

    pub fn next_ast(&mut self) -> Option<Box<ASTNode>> {
        let t = self.lexer.next_token(true);
        match t.class {
            TokenClass::Identifire(s) => Some(create_node!(ASTClass::Identifire(s))),
            TokenClass::Number(n) => Some(create_node!(ASTClass::Number(n))),
            TokenClass::Symbol(Symbol::Else) => Some(create_node!(ASTClass::Else)),
            _ => {
                panic!("unexptected token {:?}", t);
            }
        }
    }

    fn create_expression(&mut self, node: Box<ASTNode>) -> Option<Box<ASTNode>> {
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
            _ => Some(node),
        }
    }

    fn generate_mem_init_vec(&mut self) -> Vec<Box<ASTNode>> {
        let left_paren = self.lexer.next_token(true);
        if let TokenClass::Symbol(Symbol::OpeningBrace) = left_paren.class {
        } else {
            panic!("unexptected token {:?}", left_paren);
        }

        let mut args = Vec::new();
        loop {
            let token = self.lexer.next_token(true);
            match token.class {
                TokenClass::Symbol(Symbol::ClosingBrace) => {
                    if args.len() == 0 {
                        return args;
                    } else {
                        return args;
                    }
                }
                TokenClass::Symbol(Symbol::Comma) => {}
                TokenClass::Number(num) => {
                    args.push(create_node!(ASTClass::Number(num)));
                }
                _ => {
                    panic!("unexptected token {:?}", token);
                }
            }
        }
    }

    fn generate_args_vec(&mut self, is_read_left: bool) -> Vec<Box<ASTNode>> {
        if !is_read_left {
            let left_paren = self.lexer.next_token(true);
            if let TokenClass::Symbol(Symbol::LeftParen) = left_paren.class {
            } else {
                panic!("unexptected token {:?}", left_paren);
            }
        }

        let mut args = Vec::new();
        loop {
            let token = self.lexer.next_token(true);
            match token.class {
                TokenClass::Symbol(Symbol::RightParen) => {
                    return args;
                }
                TokenClass::Symbol(Symbol::Comma) => {}
                TokenClass::Identifire(id) => {
                    args.push(create_node!(ASTClass::Identifire(id)));
                }
                TokenClass::Number(num) => {
                    args.push(create_node!(ASTClass::Number(num)));
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
//             TokenClass::Macro(Macro::Else) => {
            TokenClass::Symbol(Symbol::Else) => {
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
