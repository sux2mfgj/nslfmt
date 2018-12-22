use ast::*;
use lexer::*;
use token::*;

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
        let token = self.lexer.next(true);
        match token.class {
            TokenClass::Symbol(Symbol::Sharp) => self.macro_ast(),
            TokenClass::Symbol(Symbol::Declare) => self.declare_ast(),
            TokenClass::Symbol(Symbol::Module) => self.module_ast(),
            TokenClass::CPPStyleComment(comment) => {
                create_node!(ASTClass::CPPStyleComment(comment))
            }
            TokenClass::CStyleComment(list) => {
                create_node!(ASTClass::CStyleComment(list))
            }
            TokenClass::EndOfProgram => create_node!(ASTClass::EndOfProgram),
            _ => {
                unexpected_token!(token);
            }
        }
    }

    fn macro_ast(&mut self) -> Box<ASTNode> {
        let macro_kind_token = self.lexer.next(true);
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
            TokenClass::Macro(Macro::Endif) => create_node!(ASTClass::MacroEndif),
            TokenClass::Symbol(Symbol::Else) => create_node!(ASTClass::MacroElse),
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
                let next = self.lexer.peek(true);
                if let TokenClass::Symbol(Symbol::ClosingBrace) = next.class {
                    self.lexer.next(true);
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

    fn module_ast(&mut self) -> Box<ASTNode> {
        let id_node = self.generate_id_node();
        create_node!(ASTClass::Module(id_node, self.module_block_ast()))
    }

    fn module_block_ast(&mut self) -> Box<ASTNode> {
        self.check_opening_brace();
        let mut contents_in_block = vec![];
        loop {
            if let TokenClass::Symbol(Symbol::ClosingBrace) = self.lexer.peek(true).class
            {
                self.lexer.next(true);
                return create_node!(ASTClass::Block(contents_in_block));
            }
            if let Some(block) = self.module_block_part_ast()
            {
                contents_in_block.push(block);
            }
        }
    }

    fn module_block_part_ast(&mut self) -> Option<Box<ASTNode>> {
        let t = self.lexer.next(true);
        match t.class {
            TokenClass::Symbol(Symbol::Reg) => {
                let mut reg_list = vec![];
                loop {
                    let reg_info = self.reg_definition();
                    reg_list.push(reg_info);
                    let token = self.lexer.next(true);
                    match token.class {
                        TokenClass::Symbol(Symbol::Semicolon) => {
                            break;
                        }
                        TokenClass::Symbol(Symbol::Comma) => {
                            continue;
                        }
                        _ => {
                            unexpected_token!(token);
                        }
                    }
                }
                Some(create_node!(ASTClass::Reg(reg_list)))
            }
            TokenClass::Symbol(Symbol::Wire) => {
                let mut wire_list = vec![];
                loop {
                    let next = self.lexer.next(true);
                    match next.class {
                        TokenClass::Symbol(Symbol::Semicolon) => {
                            return Some(create_node!(ASTClass::Wire(wire_list)));
                        }
                        TokenClass::Symbol(Symbol::Comma) => {
                            continue;
                        }
                        TokenClass::Identifire(id) => {
                            let id_node = create_node!(ASTClass::Identifire(id));
                            if TokenClass::Symbol(Symbol::LeftSquareBracket)
                                == self.lexer.peek(true).class
                            {
                                self.lexer.next(true);
                                let width_ast = self.expression_ast();
                                self.check_right_square_bracket();

                                wire_list.push((id_node, Some(width_ast)));
                            } else {
                                wire_list.push((id_node, None));
                            }
                        }
                        _ => {
                            unexpected_token!(next);
                        }
                    }
                }
            }
            TokenClass::Symbol(Symbol::Mem) => {
                let mut defines = vec![];
                loop {
                    let mem_info = self.mem_definition();
                    defines.push(mem_info);
                    let next = self.lexer.next(true);
                    match next.class {
                        TokenClass::Symbol(Symbol::Semicolon) => {
                            break;
                        }
                        TokenClass::Symbol(Symbol::Comma) => {
                            continue;
                        }
                        _ => {
                            unexpected_token!(next);
                        }
                    }
                }
                Some(create_node!(ASTClass::Mem(defines)))
            }
            // for behavior
            TokenClass::Identifire(id) => {
                let id_node = create_node!(ASTClass::Identifire(id));
                let next_t = self.lexer.next(true);
                match next_t.class {
                    TokenClass::Symbol(Symbol::Equal) => {
                        let expr = self.expression_ast();

                        if let TokenClass::Symbol(Symbol::LeftSquareBracket) = self.lexer.peek(true).class
                        {

                        }
                        self.check_semicolon();
                        Some(create_node!(ASTClass::Assign(id_node, expr)))
                    }
                    TokenClass::Symbol(Symbol::RegAssign) => {
                        let expr = self.expression_ast();
                        self.check_semicolon();
                        Some(create_node!(ASTClass::RegAssign(id_node, expr)))
                    }
                    TokenClass::Symbol(Symbol::LeftParen) => {
                        let args = self.generate_args_vec();
                        self.check_semicolon();
                        Some(create_node!(ASTClass::FuncCall(id_node, args,)))
                    }
                    TokenClass::UnaryOperator(op) => {
                        self.check_semicolon();
                        Some(create_node!(ASTClass::UnaryOperation(
                                id_node,
                                create_node!(ASTClass::UnaryOperator(op)))))
                    }
                    _ => {
                        unexpected_token!(next_t);
                    }
                }
            }
            TokenClass::Symbol(Symbol::ProcName) => {
                let id_node = self.generate_id_node();
                self.check_left_paren();
                let args_vec = self.generate_args_vec();
                self.check_semicolon();
                Some(create_node!(ASTClass::ProcName(id_node, args_vec,)))
            }
            TokenClass::Symbol(Symbol::StateName) => {
                let mut ids = vec![];
                loop {
                    let id_node = self.generate_id_node();
                    ids.push(id_node);

                    let n_token = self.lexer.next(true);
                    match n_token.class {
                        TokenClass::Symbol(Symbol::Semicolon) => {
                            break;
                        }
                        TokenClass::Symbol(Symbol::Comma) => {
                            continue;
                        }
                        _ => {
                            unexpected_token!(n_token);
                        }
                    }
                }
                Some(create_node!(ASTClass::StateName(ids)))
            }
            TokenClass::Symbol(Symbol::State) => {
                let id_node = self.generate_id_node();
                let block = self.module_block_ast();
                Some(create_node!(ASTClass::State(id_node, block)))
            }
            TokenClass::Symbol(Symbol::FuncSelf) => {
                let id_node = self.generate_id_node();
                let args_vec = if TokenClass::Symbol(Symbol::LeftParen)
                    == self.lexer.peek(true).class
                {
                    self.lexer.next(true);
                    Some(self.generate_args_vec())
                } else {
                    None
                };
                let return_port = self.generate_func_return();
                Some(create_node!(ASTClass::FuncSelf(id_node, args_vec, return_port,)))
            }
            TokenClass::Symbol(Symbol::Func) => {
                let id_node = self.generate_id_node();
                let mut func_name_node: Option<Box<ASTNode>> = None;
                if TokenClass::Symbol(Symbol::Dot) == self.lexer.peek(true).class {
                    self.lexer.next(true);
                    func_name_node = Some(self.generate_id_node());
                }

                let block = self.module_block_ast();
                Some(create_node!(ASTClass::Func(id_node, func_name_node, block)))
            }
            TokenClass::Symbol(Symbol::Return) => {
                let expr = self.expression_ast();
                self.check_semicolon();
                Some(create_node!(ASTClass::Return(expr)))
            }
            TokenClass::Symbol(Symbol::If) => {
                self.check_left_paren();
                let expr_ast = self.expression_ast();
                self.check_right_paren();
                let n_t = self.lexer.peek(true);
                let if_block = if let TokenClass::Symbol(Symbol::OpeningBrace) = n_t.class
                {
                    self.module_block_ast()
                } else {
                     let bb = if let Some(b) = self.module_block_part_ast() {
                        b
                     }
                     else {
                        create_node!(ASTClass::Block(vec![]))
                     };
                     bb
                };

                let else_block =
                    if TokenClass::Symbol(Symbol::Else) == self.lexer.peek(true).class {
                        self.lexer.next(true);
                        let block =
                            if let TokenClass::Symbol(Symbol::OpeningBrace) = n_t.class {
                                Some(self.module_block_ast())
                            } else {
                                self.module_block_part_ast()
                            };
                        block
                    } else {
                        None
                    };

                Some(create_node!(ASTClass::If(expr_ast, if_block, else_block)))
            }
            TokenClass::Symbol(Symbol::Any) => {
                self.check_opening_brace();

                let mut any_components = vec![];

                loop {
                    let next_t = self.lexer.peek(true);

                    match next_t.class {
                        TokenClass::Symbol(Symbol::ClosingBrace) => {
                            self.lexer.next(true);
                            break;
                        }
                        TokenClass::Symbol(Symbol::Else) => {
                            self.lexer.next(true);
                            self.check_colon();
                            let block = self.module_block_ast();
                            any_components.push((create_node!(ASTClass::Else), block));
                        }
                        _ => {
                            let ast = self.expression_ast();
                            self.check_colon();
                            let block = self.module_block_ast();
                            any_components.push((ast, block));
                        }
                    }
                }
                Some(create_node!(ASTClass::Any(any_components)))
            }
            TokenClass::UnaryOperator(op) => {
                let id = self.generate_id_node();
                self.check_semicolon();
                Some(create_node!(ASTClass::UnaryOperation(
                        create_node!(ASTClass::UnaryOperator(op)),
                        id)))
            }
            TokenClass::Symbol(Symbol::Semicolon) => {
                None
            }
            _ => {
                unexpected_token!(t);
            }
        }
    }

    fn mem_definition(
        &mut self,
    ) -> (
        Box<ASTNode>,
        Box<ASTNode>,
        Option<Box<ASTNode>>,
        Option<Vec<Box<ASTNode>>>,
    ) {
        let id_node = self.generate_id_node();
        self.check_left_square_bracket();
        let width_ast = self.expression_ast();
        self.check_right_square_bracket();

        let mut width_ast2: Option<Box<ASTNode>> = None;

        let t = self.lexer.peek(true);
        if TokenClass::Symbol(Symbol::Semicolon) == t.class {
            return (id_node, width_ast, None, None);
        }

        if TokenClass::Symbol(Symbol::LeftSquareBracket) == t.class {
            self.lexer.next(true);
            let w_ast = self.expression_ast();
            width_ast2 = Some(w_ast);
            self.check_right_square_bracket();
        }

        let next = self.lexer.peek(true);
        if TokenClass::Symbol(Symbol::Equal) == next.class {
            self.lexer.next(true);
            let initial_values = self.mem_initialize_block();
            return (id_node, width_ast, width_ast2, Some(initial_values));
        } else {
            return (id_node, width_ast, width_ast2, None);
        }
    }

    fn mem_initialize_block(&mut self) -> Vec<Box<ASTNode>> {
        self.check_opening_brace();
        let mut contents_in_block = vec![];
        loop {
            let next = self.lexer.next(true);
            match next.class {
                TokenClass::Symbol(Symbol::ClosingBrace) => {
                    break;
                }
                TokenClass::Number(num) => {
                    contents_in_block.push(create_node!(ASTClass::Number(num)));
                }
                TokenClass::Symbol(Symbol::Comma) => {
                    continue;
                }
                _ => {
                    unexpected_token!(next);
                }
            }
        }
        contents_in_block
    }

    fn reg_definition(
        &mut self,
    ) -> (Box<ASTNode>, Option<Box<ASTNode>>, Option<Box<ASTNode>>) {
        let id_node = self.generate_id_node();
        let t = self.lexer.peek(true);
        let width_ast = match t.class {
            TokenClass::Symbol(Symbol::Semicolon) => {
                return (id_node, None, None);
            }
            TokenClass::Symbol(Symbol::Comma) => {
                return (id_node, None, None);
            }
            TokenClass::Symbol(Symbol::Equal) => None,
            TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                self.lexer.next(true);
                let width_ast = self.expression_ast();
                self.check_right_square_bracket();

                let next_t = self.lexer.peek(true);
                if TokenClass::Symbol(Symbol::Semicolon) == next_t.class
                    || TokenClass::Symbol(Symbol::Comma) == next_t.class
                {
                    return (id_node, Some(width_ast), None);
                }

                if TokenClass::Symbol(Symbol::Equal) != next_t.class {
                    unexpected_token!(next_t);
                }
                Some(width_ast)
            }
            _ => {
                unexpected_token!(t);
            }
        };

        let next_t = self.lexer.next(true);
        if TokenClass::Symbol(Symbol::Equal) == next_t.class {
            let expr_ast = self.expression_ast();
            return (id_node, width_ast, Some(expr_ast));
        }

        unexpected_token!(next_t);
    }

    fn get_id_and_width(&mut self) -> (Box<ASTNode>, Option<Box<ASTNode>>) {
        let id_node = self.generate_id_node();
        match self.lexer.next(true).class {
            TokenClass::Symbol(Symbol::Semicolon) => (id_node, None),
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
        let t = self.lexer.next(true);
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
                let id_node = self.generate_id_node();
                let args_vec = if TokenClass::Symbol(Symbol::Semicolon)
                    == self.lexer.peek(true).class
                {
                    vec![]
                } else {
                    self.check_left_paren();
                    self.generate_args_vec()
                };

                let return_port = self.generate_func_return();
                create_node!(ASTClass::FuncIn(id_node, args_vec, return_port,))
            }
            TokenClass::Symbol(Symbol::FuncOut) => {
                let id_node = self.generate_id_node();
                let args_vec = if TokenClass::Symbol(Symbol::Semicolon)
                    == self.lexer.peek(true).class
                {
                    vec![]
                } else {
                    self.check_left_paren();
                    self.generate_args_vec()
                };

                let return_port = self.generate_func_return();
                create_node!(ASTClass::FuncOut(id_node, args_vec, return_port,))
            }
            _ => {
                unexpected_token!(t);
            }
        };
    }

    fn to_node(&self, t: Token) -> Box<ASTNode> {
        return match t.class {
            TokenClass::Number(num) => create_node!(ASTClass::Number(num)),
            TokenClass::Identifire(id) => create_node!(ASTClass::Identifire(id)),
            _ => {
                unexpected_token!(t);
            }
        };
    }

    fn expression_ast(&mut self) -> (Box<ASTNode>) {
        let t = self.lexer.next(true);
        let left = match t.class
        {
            TokenClass::UnaryOperator(op) => {
                let id = self.generate_id_node();
                create_node!(ASTClass::UnaryOperation(
                        create_node!(ASTClass::UnaryOperator(op)),
                        id))
            }
            _ =>
            {
                self.to_node(t)
            }
        };

        let n_token = self.lexer.peek(true);

        let left_node = match n_token.class
        {
            TokenClass::Symbol(Symbol::LeftParen) => {
                self.lexer.next(true);
                let args = self.generate_args_vec();
                create_node!(ASTClass::FuncCall(left, args))
            }
            TokenClass::UnaryOperator(op) => {
                self.lexer.next(true);
                create_node!(ASTClass::UnaryOperation(
                        left,
                        create_node!(ASTClass::UnaryOperator(op))))
            }
            _ => {
                left
            }
        };

        let nn_token = self.lexer.peek(true);
        if let TokenClass::Operator(op) = nn_token.class {
            self.lexer.next(true);
            return create_node!(ASTClass::Expression(
                left_node,
                create_node!(ASTClass::Operator(op)),
                self.expression_ast()
            ));
        }
        left_node
    }

    fn generate_id_node(&mut self) -> Box<ASTNode> {
        let id_token = self.lexer.next(true);
        if let TokenClass::Identifire(id_str) = id_token.class {
            return create_node!(ASTClass::Identifire(id_str));
        } else {
            unexpected_token!(id_token)
        }
    }

    fn generate_args_vec(&mut self) -> Vec<Box<ASTNode>> {
        let mut args = vec![];
        loop {
            let token = self.lexer.next(true);
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
        let colon_token = self.lexer.peek(true);

        return if TokenClass::Symbol(Symbol::Colon) == colon_token.class {
            self.lexer.next(true);
            let port_id = self.lexer.next(true);

            if let TokenClass::Identifire(id_str) = port_id.class {
                self.check_semicolon();
                Some(create_node!(ASTClass::Identifire(id_str)))
            } else {
                unexpected_token!(port_id);
            }
        } else {
            self.check_semicolon();
            None
        };
    }

    fn generate_string_until_nl(&mut self) -> Option<String> {
        let mut t_list: Vec<Token> = vec![];
        loop {
            let t = self.lexer.next(false);
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
        let path_token = self.lexer.next(true);
        if let TokenClass::String(id_str) = path_token.class {
            return create_node!(ASTClass::String(id_str));
        }
        unexpected_token!(path_token);
    }

    fn check_opening_brace(&mut self) {
        let token = self.lexer.next(true);
        if TokenClass::Symbol(Symbol::OpeningBrace) != token.class {
            unexpected_token!(token);
        }
    }

    fn check_right_square_bracket(&mut self) {
        let token = self.lexer.next(true);
        if TokenClass::Symbol(Symbol::RightSquareBracket) != token.class {
            unexpected_token!(token);
        }
    }

    fn check_left_square_bracket(&mut self) {
        let token = self.lexer.next(true);
        if TokenClass::Symbol(Symbol::LeftSquareBracket) != token.class {
            unexpected_token!(token);
        }
    }

    fn check_semicolon(&mut self) {
        let token = self.lexer.next(true);
        if TokenClass::Symbol(Symbol::Semicolon) != token.class {
            unexpected_token!(token);
        }
    }

    fn check_left_paren(&mut self) {
        let token = self.lexer.next(true);
        if TokenClass::Symbol(Symbol::LeftParen) != token.class {
            unexpected_token!(token);
        }
    }

    fn check_right_paren(&mut self) {
        let token = self.lexer.next(true);
        if TokenClass::Symbol(Symbol::RightParen) != token.class {
            unexpected_token!(token);
        }
    }

    fn check_colon(&mut self) {
        let token = self.lexer.next(true);
        if TokenClass::Symbol(Symbol::Colon) != token.class {
            unexpected_token!(token);
        }
    }
}
