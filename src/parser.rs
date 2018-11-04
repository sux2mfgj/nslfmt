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
        let id_token = self.lexer.next_token(true);
        let brace_token = self.lexer.next_token(true);

        if let (
            TokenClass::Identifire(id_str),
            TokenClass::Symbol(Symbol::OpeningBrace),
            ) = (&id_token.class, &brace_token.class)
        {
            let mut contents_in_block = Vec::new();
            loop {
                let next_token = self.lexer.next_token(false);
                match next_token.class {
                    TokenClass::Symbol(Symbol::ClosingBrace) => {
                        return create_node!(ASTClass::Declare(
                                create_node!(ASTClass::Identifire(id_str.to_string())),
                                create_node!(ASTClass::Block(contents_in_block))));
                    }
                    _ => {
                        contents_in_block.push(self.declare_block_part_ast(next_token));
                    }
                }
            }
        }
        else
        {
            panic!("unexpected token. it should be '<id>' and '{{' but {} and {}."
                   , id_token, brace_token);
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
                            let (width_ast, token) = self.width_expression_ast();
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
                            let (width_ast, token) = self.width_expression_ast();
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
                            let (width_ast, token) = self.width_expression_ast();
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
                    let args_vec = self.generate_args_vec();
                    let return_port = self.generate_func_return();
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
                    let args_vec = self.generate_args_vec();
                    let return_port = self.generate_func_return();
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

    fn width_expression_ast(&mut self) -> (Box<ASTNode>, Token) {
        let first_token = self.lexer.next_token(true);
        let second_token = self.lexer.next_token(true);
        match first_token.class {
            TokenClass::Number(num) => {
                if let TokenClass::Operator(op) = second_token.class {
                    let (right_ast, token) = self.width_expression_ast();
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
                    let (right_ast, token) = self.width_expression_ast();
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
                else {
                    return (
                        create_node!(ASTClass::Identifire(id_str)),
                        second_token,
                        )
                }
            }
            _ => {
                unexpected_token!(first_token);
            }
        }
    }

    fn check_semicolon(&mut self, semicolon_token: Token) {
        //let semicolon_token = self.lexer.next_token(true);
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

    fn check_right_square_bracket(&mut self, token: Token) {
        if TokenClass::Symbol(Symbol::RightSquareBracket) != token.class {
            unexpected_token!(token);
        }
    }

    fn generate_args_vec(&mut self) -> Vec<Box<ASTNode>> {
        let left_paren = self.lexer.next_token(true);
        if TokenClass::Symbol(Symbol::LeftParen) != left_paren.class {
            unexpected_token!(left_paren);
        }
        let mut args = Vec::new();
        loop {
            let token = self.lexer.next_token(true);
            match token.class {
                TokenClass::Symbol(Symbol::RightParen) => {
                    break;
                }
                TokenClass::Identifire(id_str) => {
                    args.push(create_node!(ASTClass::Identifire(id_str)));
                }
                _ => {
                    unexpected_token!(token);
                }
            }
        }

        args
    }

    fn generate_func_return(&mut self) -> Option<Box<ASTNode>> {
        let colon_token = self.lexer.next_token(true);
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
