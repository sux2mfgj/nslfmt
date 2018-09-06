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
    nodes: Vec<ASTNode>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer: lexer,
            nodes: Vec::new(),
        }
    }

    pub fn generate_ast(&mut self) -> Result<ASTNode, ASTError> {
        let mut l = self.lexer.get_next_token();
        match l.class {
            //TokenClass::Symbol(Symbol::Declare) => {
            //}
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
}

#[cfg(test)]
mod parser_test {

    use super::*;

    #[test]
    fn end_of_program() {
        let mut b = "".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        assert_eq!(p.generate_ast().err(), Some(ASTError::EndOfProgram));
    }

    #[test]
    fn unexptected_token() {
        let mut b = "a".as_bytes();
        let mut l = Lexer::new(&mut b);
        let mut p = Parser::new(&mut l);

        assert_eq!(p.generate_ast().err(), Some(ASTError::UnExpectedToken));
    }

}

