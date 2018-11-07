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
        let token = self.lexer.next_token(false);
        match token.class {
            TokenClass::Symbol(Symbol::Sharp) => self.macro_ast(),
            TokenClass::Symbol(Symbol::Declare) => self.declare_ast(),
            TokenClass::Symbol(Symbol::Module) => self.module_ast(),
            TokenClass::EndOfProgram => create_node!(ASTClass::EndOfProgram),
            TokenClass::Newline => create_node!(ASTClass::Newline),
            TokenClass::CPPStyleComment(list) => create_node!(ASTClass::CPPStyleComment(list)),
            _ => {
                unexpected_token!(token);
            }
        }
    }

    fn declare_ast(&mut self) -> Box<ASTNode> {
        // <identifire>
        let id_node = self.generate_id_node();
        let opening_brace_token = self.lexer.next_token(true);
        self.check_opening_brace(opening_brace_token);
        let mut contents_in_block = vec![];
        loop {
            let next_token = self.lexer.next_token(false);
            match next_token.class {
                TokenClass::Symbol(Symbol::ClosingBrace) => {
                    return create_node!(ASTClass::Declare(
                            id_node,
                            create_node!(ASTClass::Block(contents_in_block))));
                }
                _ => {
                    contents_in_block.push(self.declare_block_part_ast(next_token));
                }
            }
        }
    }

    fn declare_block_part_ast(&mut self, t: Token) -> Box<ASTNode> {
        match t.class {
            TokenClass::Newline => {
                return create_node!(ASTClass::Newline);
            }
            TokenClass::Identifire(id_str) => {
                return create_node!(ASTClass::Identifire(id_str));
            }
            TokenClass::Symbol(Symbol::Input) => {
                let id_token = self.lexer.next_token(true);
                if let TokenClass::Identifire(id_str) = &id_token.class
                {
                    let next_token = self.lexer.next_token(true);
                    match next_token.class
                    {
                        TokenClass::Symbol(Symbol::Semicolon) => {
                            return create_node!(ASTClass::Input(
                                    create_node!(ASTClass::Identifire(id_str.to_string())),
                                    None));
                        }
                        TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                            let next_t = self.lexer.next_token(true);
                            let (width_ast, token) = self.expression_ast(next_t);
                            self.check_right_square_bracket(token);
                            let semicolon_token = self.lexer.next_token(true);
                            self.check_semicolon(semicolon_token);
                            return create_node!(
                                ASTClass::Input(
                                    create_node!(ASTClass::Identifire(id_str.to_string())),
                                    Some(width_ast)));
                        }
                        _ => {
                            unexpected_token!(next_token);
                        }
                    }
                }
                else {
                    unexpected_token!(id_token);
                }
            }
            TokenClass::Symbol(Symbol::Output) => {
                let id_token = self.lexer.next_token(true);
                if let TokenClass::Identifire(id_str) = &id_token.class
                {
                    let next_token = self.lexer.next_token(true);
                    match next_token.class
                    {
                        TokenClass::Symbol(Symbol::Semicolon) => {
                            return create_node!(ASTClass::Output(
                                    create_node!(ASTClass::Identifire(id_str.to_string())),
                                    None));
                        }
                        TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                            let next_t = self.lexer.next_token(true);
                            let (width_ast, token) = self.expression_ast(next_t);

                            self.check_right_square_bracket(token);
                            let semicolon_token = self.lexer.next_token(true);
                            self.check_semicolon(semicolon_token);
                            return create_node!(
                                ASTClass::Output(
                                    create_node!(ASTClass::Identifire(id_str.to_string())),
                                    Some(width_ast)));
                        }
                        _ => {
                            unexpected_token!(next_token);
                        }
                    }
                }
                else {
                    unexpected_token!(id_token);
                }
            }
            TokenClass::Symbol(Symbol::InOut) => {
                let id_token = self.lexer.next_token(true);
                if let TokenClass::Identifire(id_str) = &id_token.class
                {
                    let next_token = self.lexer.next_token(true);
                    match next_token.class
                    {
                        TokenClass::Symbol(Symbol::Semicolon) => {
                            return create_node!(ASTClass::InOut(
                                    create_node!(ASTClass::Identifire(id_str.to_string())),
                                    None));
                        }
                        TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                            let next_t = self.lexer.next_token(true);
                            let (width_ast, token) = self.expression_ast(next_t);

                            self.check_right_square_bracket(token);
                            let semicolon_token = self.lexer.next_token(true);
                            self.check_semicolon(semicolon_token);
                            return create_node!(
                                ASTClass::InOut(
                                    create_node!(ASTClass::Identifire(id_str.to_string())),
                                    Some(width_ast)));
                        }
                        _ => {
                            unexpected_token!(next_token);
                        }
                    }
                }
                else {
                    unexpected_token!(id_token);
                }
            }
            TokenClass::Symbol(Symbol::FuncIn) => {
                let id_token = self.lexer.next_token(true);
                if let TokenClass::Identifire(id_str) = &id_token.class
                {
                    let left_paren = self.lexer.next_token(true);
                    let (args_vec, next_t) = self.generate_args_vec(left_paren);

//                     let (args_vec, next_t) = self.generate_args_vec();
                    let return_port = self.generate_func_return(next_t);
                    return create_node!(ASTClass::FuncIn(
                            create_node!(ASTClass::Identifire(id_str.to_string())),
                            args_vec,
                            return_port,
                            ));
                }
                else {
                    unexpected_token!(id_token);
                }
            }
            TokenClass::Symbol(Symbol::FuncOut) => {
                let id_token = self.lexer.next_token(true);
                if let TokenClass::Identifire(id_str) = &id_token.class
                {
                    let left_paren = self.lexer.next_token(true);
                    let (args_vec, next_t) = self.generate_args_vec(left_paren);

//                     let (args_vec, next_t) = self.generate_args_vec();
                    let return_port = self.generate_func_return(next_t);
                    return create_node!(ASTClass::FuncOut(
                            create_node!(ASTClass::Identifire(id_str.to_string())),
                            args_vec,
                            return_port,
                            ));
                }
                else {
                    unexpected_token!(id_token);
                }
            }
            _ => {
                unexpected_token!(t);
            }
        }
    }

    fn module_ast(&mut self) -> Box<ASTNode> {
        let id_node = self.generate_id_node();
        let opening_brace_token = self.lexer.next_token(true);
        self.check_opening_brace(opening_brace_token);
        let mut contents_in_block = vec![];
        loop {
            let next_token = self.lexer.next_token(false);
            match next_token.class {
                TokenClass::Symbol(Symbol::ClosingBrace) => {
                    return create_node!(
                        ASTClass::Module(
                            id_node,
                            create_node!(ASTClass::Block(contents_in_block))
                            )
                        );
                }
                _ => {
                    contents_in_block.push(self.module_block_part_ast(next_token));
                }
            }
        }
    }

    fn module_behavior_ast(&mut self, opening_brace_token: Token) -> Box<ASTNode> {
        self.check_opening_brace(opening_brace_token);
        let mut contents_in_block = vec![];
        loop {
            let next_token = self.lexer.next_token(false);
            match next_token.class {
                TokenClass::Symbol(Symbol::ClosingBrace) => {
                    return create_node!(ASTClass::Block(contents_in_block));
                }
                _ => {
                    contents_in_block.push(self.module_block_part_ast(next_token));
                }
            }
        }
    }

    fn module_block_part_ast(&mut self, t: Token) -> Box<ASTNode> {
        match t.class {
            TokenClass::Newline => {
                return create_node!(ASTClass::Newline);
            }
            TokenClass::Symbol(Symbol::Wire) => {
                let mut wire_list = vec![];
                loop {
                    let id_node = self.generate_id_node();
                    let next_token = self.lexer.next_token(true);
                    match next_token.class {
                        TokenClass::Symbol(Symbol::Semicolon) => {
                            wire_list.push((id_node, None));
                            return create_node!(ASTClass::Wire(wire_list));
                        }
                        TokenClass::Symbol(Symbol::Comma) => {
                            wire_list.push((id_node, None));
                            continue;
                        }
                        TokenClass::Symbol(Symbol::LeftSquareBracket) => {
                            let next_t = self.lexer.next_token(true);
                            let (width_ast, token) = self.expression_ast(next_t);

                            self.check_right_square_bracket(token);
                            wire_list.push((id_node, Some(width_ast)));

                            let next_t = self.lexer.next_token(true);
                            if TokenClass::Symbol(Symbol::Semicolon) == next_t.class
                            {
                                return create_node!(ASTClass::Wire(wire_list));
                            }
                            if TokenClass::Symbol(Symbol::Comma) == next_t.class
                            {
                                continue;
                            }
                            unexpected_token!(next_t);
                        }
                        _ => {
                            unexpected_token!(next_token);
                        }
                    }
                }
            }
            TokenClass::Symbol(Symbol::Reg) => {
                let mut reg_list = vec![];
                loop {
                    let (reg_info, token) = self.reg_definition();
                    reg_list.push(reg_info);
                    match token.class
                    {
                        TokenClass::Symbol(Symbol::Semicolon) =>
                        {
                            break;
                        }
                        TokenClass::Symbol(Symbol::Comma) =>
                        {
                            continue;
                        }
                        _ =>
                        {
                            unexpected_token!(token);
                        }
                    }
                }
                return create_node!(ASTClass::Reg(reg_list))
            }
            TokenClass::Symbol(Symbol::FuncSelf) => {
                let id_node = self.generate_id_node();
                let left_paren = self.lexer.next_token(true);
                let (args_vec, next_t) = self.generate_args_vec(left_paren);

//                 let (args_vec, next_t) = self.generate_args_vec();
                let return_port = self.generate_func_return(next_t);
                return create_node!(
                    ASTClass::FuncSelf(
                        id_node,
                        args_vec,
                        return_port,
                        )
                    );
            }
            TokenClass::Symbol(Symbol::ProcName) => {
                let id_node = self.generate_id_node();
                let left_paren = self.lexer.next_token(true);
                let (args_vec, next_t) = self.generate_args_vec(left_paren);

//                 let (args_vec, next_t) = self.generate_args_vec();
                self.check_semicolon(next_t);
                return create_node!(
                    ASTClass::ProcName(
                        id_node,
                        args_vec,
                        )
                    );
            }
            TokenClass::Symbol(Symbol::StateName) => {
                let mut ids = vec![];
                loop {
                    let token = self.lexer.next_token(true);
                    if let TokenClass::Identifire(id_str) = token.class
                    {
                        ids.push(create_node!(ASTClass::Identifire(id_str)));

                        let n_token = self.lexer.next_token(true);
                        match n_token.class
                        {
                            TokenClass::Symbol(Symbol::Semicolon) =>
                            {
                                break;
                            }
                            TokenClass::Symbol(Symbol::Comma) =>
                            {
                                continue;
                            }
                            _ => {
                                unexpected_token!(n_token);
                            }
                        }
                    }
                    else
                    {
                        unexpected_token!(token);
                    }
                }
                return create_node!(ASTClass::StateName(ids));
            }
            TokenClass::Symbol(Symbol::Mem) => {
                let mut defines = vec![];
                loop {
                    let (mem_info, next_token) = self.mem_definition();
                    defines.push(mem_info);
                    match next_token.class
                    {
                        TokenClass::Symbol(Symbol::Semicolon) =>
                        {
                            break;
                        }
                        TokenClass::Symbol(Symbol::Comma) =>
                        {
                            continue;
                        }
                        _ =>
                        {
                            unexpected_token!(next_token);
                        }
                    }
                }
                return create_node!(ASTClass::Mem(defines));
            }
            TokenClass::Identifire(id_str) => {
                let id_node = create_node!(ASTClass::Identifire(id_str));
                let next_t = self.lexer.next_token(true);
                match next_t.class
                {
                    TokenClass::Symbol(Symbol::Equal) => {
                        let next_t = self.lexer.next_token(true);
                        let (expr, nn_t) = self.expression_ast(next_t);
                        self.check_semicolon(nn_t);
                        return create_node!(ASTClass::Assign(id_node, expr))
                    }
                    TokenClass::Symbol(Symbol::RegAssign) => {
                        let next_t = self.lexer.next_token(true);
                        let (expr, nn_t) = self.expression_ast(next_t);
                        self.check_semicolon(nn_t);
                        return create_node!(ASTClass::RegAssign(id_node, expr));
                    }
                    TokenClass::Symbol(Symbol::LeftParen) => {
                        let (args, n_t) = self.generate_args_vec(next_t);
                        self.check_semicolon(n_t);
                        return create_node!(
                            ASTClass::FuncCall(
                                id_node,
                                args,
                                )
                            );
                    }
                    _ => {
                        unexpected_token!(next_t);
                    }
                }
            }
            TokenClass::Symbol(Symbol::Func) => {
                let id_node = self.generate_id_node();
                let mut func_name_node: Option<Box<ASTNode>> = None;
                let mut dot = self.lexer.next_token(true);
                if TokenClass::Symbol(Symbol::Dot) == dot.class
                {
                    func_name_node = Some(self.generate_id_node());
                    dot = self.lexer.next_token(true);
                }

                let block = self.module_behavior_ast(dot);
                return create_node!(ASTClass::Func(id_node, func_name_node, block));
            }
            TokenClass::Symbol(Symbol::Return) => {
                let next_t = self.lexer.next_token(true);
                let (expr, nn_t) = self.expression_ast(next_t);
                self.check_semicolon(nn_t);
                return create_node!(ASTClass::Return(expr));
            }
            TokenClass::Symbol(Symbol::Any) => {
                let opening_brace_token = self.lexer.next_token(true);
                self.check_opening_brace(opening_brace_token);

                let mut any_components = vec![];

                loop {
                    let next_t = self.lexer.next_token(true);

                    match next_t.class
                    {
                        TokenClass::Symbol(Symbol::ClosingBrace) => {
                            break;
                        }
                        TokenClass::Symbol(Symbol::Else) => {
                            let nn_t = self.lexer.next_token(true);
                            self.check_colon(nn_t);
                            let opening_brace_token = self.lexer.next_token(true);
                            let block = self.module_behavior_ast(opening_brace_token);
                            any_components.push((create_node!(ASTClass::Else), block));
                        }
                        _ => {
                            let (ast, nn_t) = self.expression_ast(next_t);
                            let opening_brace_token = self.lexer.next_token(true);
                            let block = self.module_behavior_ast(opening_brace_token);
                            any_components.push((ast, block));
                        }
                    }
                }
                return create_node!(ASTClass::Any(any_components));
            }
            _ => {
                unexpected_token!(t);
            }
        }
    }

    fn reg_definition(&mut self)
        -> ((Box<ASTNode>, Option<Box<ASTNode>>, Option<Box<ASTNode>>), Token)
    {
        let id_node = self.generate_id_node();
        let t = self.lexer.next_token(true);
        match t.class
        {
            TokenClass::Symbol(Symbol::Semicolon) =>
            {
                return ((id_node, None, None), t);
            }
            TokenClass::Symbol(Symbol::Comma) => {
                return ((id_node, None, None), t);
            }
            TokenClass::Symbol(Symbol::Equal) => {
                let n_t = self.lexer.next_token(true);
                let (expr_ast, nn_t) = self.expression_ast(n_t);
                if TokenClass::Symbol(Symbol::Semicolon) == nn_t.class ||
                    TokenClass::Symbol(Symbol::Comma) == nn_t.class
                {
                    return ((id_node, None, Some(expr_ast)), nn_t)
                }
                unexpected_token!(nn_t);
            }
            TokenClass::Symbol(LeftSquareBracket) => {
                //let (width_ast, token) = self.width_expression_ast();
                let next_t = self.lexer.next_token(true);
                let (width_ast, token) = self.expression_ast(next_t);
                self.check_right_square_bracket(token);

                let next_t = self.lexer.next_token(true);
                if TokenClass::Symbol(Symbol::Semicolon) == next_t.class ||
                    TokenClass::Symbol(Symbol::Comma) == next_t.class
                {
                    return ((id_node, Some(width_ast), None), next_t)
                }

                if TokenClass::Symbol(Symbol::Equal) == next_t.class
                {
                    let next_t = self.lexer.next_token(true);
                    let (expr_ast, nn_t) = self.expression_ast(next_t);
                    return ((id_node, Some(width_ast), Some(expr_ast)), nn_t);
                }
                unexpected_token!(next_t);
            }
            _ => {
                unexpected_token!(t);
            }
        }
    }

    fn mem_definition(&mut self) ->
        ((Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>, Option<Vec<Box<ASTNode>>>),
         Token)
    {
        let id_node = self.generate_id_node();
        let left_square_token = self.lexer.next_token(true);
        if TokenClass::Symbol(Symbol::LeftSquareBracket) != left_square_token.class
        {
            unexpected_token!(left_square_token);
        }
        let next_t = self.lexer.next_token(true);
        let (width_ast, token) = self.expression_ast(next_t);

        self.check_right_square_bracket(token);
        let mut width_ast2: Option<Box<ASTNode>> = None;
        let mut init_block: Option<Vec<Box<ASTNode>>> = None;

        let mut t = self.lexer.next_token(true);
        if TokenClass::Symbol(Symbol::Semicolon) == t.class
        {
            return ((id_node, width_ast, None, None), t)
        }

        if TokenClass::Symbol(Symbol::LeftSquareBracket) == t.class
        {
            let next_t = self.lexer.next_token(true);
            let (w_ast, token) = self.expression_ast(next_t);
            width_ast2 = Some(w_ast);
            self.check_right_square_bracket(token);

            t = self.lexer.next_token(true);
            if TokenClass::Symbol(Symbol::Semicolon) == t.class
            {
                return ((id_node, width_ast, width_ast2, None), t)
            }
        }

        if TokenClass::Symbol(Symbol::Equal) == t.class
        {
            let initial_values = self.mem_initialize_block();
            return (
                    (id_node, width_ast, width_ast2, Some(initial_values)),
                    self.lexer.next_token(true),
                );

        }
        not_implemented!();
    }

    fn mem_initialize_block(&mut self) -> Vec<Box<ASTNode>>
    {
        let opening_brace_token = self.lexer.next_token(true);
        self.check_opening_brace(opening_brace_token);
        let mut contents_in_block = vec![];
        loop {
            let next_token = self.lexer.next_token(false);
            match next_token.class {
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
                    unexpected_token!(next_token);
                }
            }
        }
        contents_in_block
    }

    fn macro_ast(&mut self) -> Box<ASTNode> {
        let macro_kind_token = self.lexer.next_token(true);
        match macro_kind_token.class {
            TokenClass::Macro(Macro::Include) => {
                return create_node!(ASTClass::MacroInclude(
                        self.generate_path_node()));
            }
            TokenClass::Macro(Macro::Undef) => {
                let id = self.generate_id_node();
                return create_node!(ASTClass::MacroUndef(id));
            }
            TokenClass::Macro(Macro::Ifdef) => {
                let id = self.generate_id_node();
                return create_node!(ASTClass::MacroIfdef(id));
            }
            TokenClass::Macro(Macro::Ifndef) => {
                let id = self.generate_id_node();
                return create_node!(ASTClass::MacroIfndef(id));
            }
            TokenClass::Macro(Macro::Endif) => {
                return create_node!(ASTClass::MacroEndif);
            }
            TokenClass::Symbol(Symbol::Else) => {
                return create_node!(ASTClass::MacroElse);
            }
            TokenClass::Macro(Macro::Define) => {
                let id = self.generate_id_node();
                let value = self.generate_string_until_nl();
                return create_node!(ASTClass::MacroDefine(
                        id,
                        value
                        ));
            }
            _ => {
                unexpected_token!(macro_kind_token);
            }
        }
    }

    /*
     * utility functions
     */

    fn expression_ast(&mut self, first_token: Token) -> (Box<ASTNode>, Token) {
        let second_token = self.lexer.next_token(true);
        match first_token.class {
            TokenClass::Number(num) => {
                if let TokenClass::Operator(op) = second_token.class {
                    let next_t = self.lexer.next_token(true);
                    let (right_ast, token) = self.expression_ast(next_t);
                    return (
                        create_node!(
                            ASTClass::Expression(
                                create_node!(ASTClass::Number(num)),
                                create_node!(ASTClass::Operator(op)),
                                right_ast,
                                )),
                            token
                        );

                }
                else {
                    return (
                        create_node!(ASTClass::Number(num)),
                        second_token,
                        )
                }
            }
            TokenClass::Identifire(id_str) => {
                if let TokenClass::Operator(op) = second_token.class {
                    let next_t = self.lexer.next_token(true);
                    let (right_ast, token) = self.expression_ast(next_t);
                    return (
                        create_node!(
                            ASTClass::Expression(
                                create_node!(ASTClass::Identifire(id_str)),
                                create_node!(ASTClass::Operator(op)),
                                right_ast,
                                )),
                            token
                        );
                }
                else if let TokenClass::Symbol(Symbol::LeftParen) = second_token.class {
                    let (args_vec, next_t) = self.generate_args_vec(second_token);
                    let func_call_node = create_node!(
                            ASTClass::FuncCall(
                                create_node!(ASTClass::Identifire(id_str)),
                                args_vec
                                )
                            );

                    if let TokenClass::Operator(op) = next_t.class {
                        let nn_t = self.lexer.next_token(true);
                        let (right_ast, nnn_t) = self.expression_ast(nn_t);
                        return (
                            create_node!(
                                ASTClass::Expression(
                                    func_call_node,
                                    create_node!(ASTClass::Operator(op)),
                                    right_ast
                                ))
                            , nnn_t);
                    }
                    return (func_call_node, next_t);
                }
                else {
                    return (
                        create_node!(ASTClass::Identifire(id_str)),
                        second_token,
                        )
                }
            }
            TokenClass::Symbol(Symbol::ClosingBrace) => {
                not_implemented!();
            }
            _ => {
                unexpected_token!(first_token);
            }
        }
    }


    fn check_semicolon(&mut self, semicolon_token: Token) {
        if TokenClass::Symbol(Symbol::Semicolon) != semicolon_token.class
        {
            unexpected_token!(semicolon_token);
        }
    }

    fn check_colon(&mut self, colon_token: Token) {
        if TokenClass::Symbol(Symbol::Colon) != colon_token.class {
            unexpected_token!(colon_token);
        }
    }

    fn check_opening_brace(&mut self, opening_brace_token: Token) {
        if TokenClass::Symbol(Symbol::OpeningBrace) != opening_brace_token.class
        {
            unexpected_token!(opening_brace_token);
        }
    }

    fn check_right_square_bracket(&mut self, token: Token) {
        if TokenClass::Symbol(Symbol::RightSquareBracket) != token.class {
            unexpected_token!(token);
        }
    }

    fn generate_args_vec(&mut self, left_paren: Token) -> (Vec<Box<ASTNode>>, Token) {
//         let left_paren = self.lexer.next_token(true);
        let mut args = vec![];
        if TokenClass::Symbol(Symbol::LeftParen) != left_paren.class {
            // probably, the left_paren is semicolon token
            return (args, left_paren);
        }
        loop {
            let token = self.lexer.next_token(true);
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
                _ => {
                    unexpected_token!(token);
                }
            }
        }

        (args, self.lexer.next_token(true))
    }

    fn generate_func_return(&mut self, colon_token: Token) -> Option<Box<ASTNode>> {
        let mut return_port: Option<Box<ASTNode>> = None;
        if TokenClass::Symbol(Symbol::Colon) == colon_token.class {
            let port_id = self.lexer.next_token(true);

            if let TokenClass::Identifire(id_str) = port_id.class {
                return_port = Some(create_node!(ASTClass::Identifire(id_str)));
            }
            else {
                unexpected_token!(port_id);
            }
            let semicolon = self.lexer.next_token(true);
            self.check_semicolon(semicolon);
        }
        else {
            self.check_semicolon(colon_token);
        }
        return_port
    }

    fn generate_path_node(&mut self) -> Box<ASTNode> {
        let path_token = self.lexer.next_token(true);
        if let TokenClass::String(id_str) = path_token.class {
            return create_node!(ASTClass::String(id_str));
        }
        unexpected_token!(path_token);
    }

    fn generate_id_node(&mut self) -> Box<ASTNode> {
        let id_token = self.lexer.next_token(true);
        if let TokenClass::Identifire(id_str) = id_token.class {
            return create_node!(ASTClass::Identifire(id_str));
        }
        else {
            unexpected_token!(id_token)
        }
    }

    fn generate_string_until_nl(&mut self) -> Option<String> {
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
                        None => {
                            return None
                        }
                    }
                }
                _ => {
                    t_list.push(t);
                }
            }
        }
    }
}
