extern crate nslfmt;

use nslfmt::lexer::Lexer;
use nslfmt::token::*;
use std::fs::File;
use std::io::BufReader;

#[test]
fn create_instance_with_string() {
    let mut b = "declare hello {input ok; func_in(ok);}".as_bytes();
    let _l = Lexer::new(&mut b);
}

#[test]
fn create_instance_with_file() {
    let mut f = BufReader::new(File::open("nsl_samples/fetch.nsl").unwrap());
    let _l = Lexer::new(&mut f);
}

#[test]
fn get_token_eop() {
    let mut b = "".as_bytes();
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::from((TokenClass::EndOfProgram, 1)));
}

#[test]
fn get_token_new_line() {
    let mut b = "\n".as_bytes();
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::from((TokenClass::EndOfProgram, 2)));
}

#[test]
fn peek_00() {
    let mut b = "\nhello\n".as_bytes();
    let mut l = Lexer::new(&mut b);
    // TODO
    assert_eq!(
        l.peek(true),
        Token::from((TokenClass::Identifire("hello".to_string()), 2))
    );
}

#[test]
fn pass_00() {
    let mut b = "\nhello\n".as_bytes();
    let mut l = Lexer::new(&mut b);
    // TODO
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("hello".to_string()), 2))
    );
}

#[test]
fn pass_newlines() {
    let mut b = "\n".as_bytes();
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::from((TokenClass::EndOfProgram, 2)));
}

#[test]
fn get_token_str() {
    let mut b = "declare".as_bytes();
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Declare), 1));
}

#[test]
fn get_token_file() {
    let mut f = BufReader::new(File::open("nsl_samples/declare.nsl").unwrap());
    let mut l = Lexer::new(&mut f);
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Declare), 1));
}

#[test]
fn braces_and_newline() {
    let mut b = "declare {  \n }".as_bytes();
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Declare), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 2)
    );
}

#[test]
fn declare_with_input() {
    let mut b = BufReader::new(File::open("nsl_samples/declare_01.nsl").unwrap());
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Declare), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("hello".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 2)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Input), 3));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("ok".to_string()), 3)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 3)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 4)
    );
}

#[test]
fn declare_func_in() {
    let mut b = BufReader::new(File::open("nsl_samples/declare_02.nsl").unwrap());
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Declare), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("hello_google2".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 2)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Input), 3));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("ok".to_string()), 3)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 3)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::FuncIn), 4));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("sugoi".to_string()), 4)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::LeftParen), 4)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("ok".to_string()), 4)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::RightParen), 4)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 4)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 5)
    );
}

#[test]
fn declare_func_out() {
    let mut b = BufReader::new(File::open("nsl_samples/declare_03.nsl").unwrap());
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Declare), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("hel".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 2)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Input), 3));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("ok".to_string()), 3)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 3)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Input), 4));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("ggrks".to_string()), 4)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 4)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Output), 5));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("jk".to_string()), 5)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 5)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::FuncIn), 7));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("sugoi".to_string()), 7)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::LeftParen), 7)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("ok".to_string()), 7)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::RightParen), 7)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 7)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::FuncOut), 8));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("majika".to_string()), 8)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::LeftParen), 8)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("jk".to_string()), 8)
    );

    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::RightParen), 8)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Colon), 8));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("ggrks".to_string()), 8)
    );

    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 8)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 9)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 10));
}

#[test]
fn number() {
    let mut b = "declare ok {input a[12];}".as_bytes();
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Declare), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("ok".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 1)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Input), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("a".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::LeftSquareBracket), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Number("12".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::RightSquareBracket), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 1)
    );
}

#[test]
fn declare_04() {
    let mut b = BufReader::new(File::open("nsl_samples/declare_04.nsl").unwrap());
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Declare), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("test".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 2)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Input), 3));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("aa".to_string()), 3)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 3)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Input), 4));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("jk".to_string()), 4)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 4)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::FuncIn), 6));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("ok".to_string()), 6)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::LeftParen), 6)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("aa".to_string()), 6)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Comma), 6));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("jk".to_string()), 6)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::RightParen), 6)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 6)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 7)
    );
}

#[test]
fn macro_include() {
    let mut b = "#include \"hello.h\"\ndeclare ok {}".as_bytes();
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::Macro(Macro::Include), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::String("hello.h".to_string()), 1)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Declare), 2));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("ok".to_string()), 2)
    );
}

#[test]
fn macro_undef() {
    let mut b = "#undef aaaa".as_bytes();
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::Macro(Macro::Undef), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("aaaa".to_string()), 1)
    );
}

#[test]
fn macro_ifdef() {
    let mut b = "#ifdef aaaa".as_bytes();
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::Macro(Macro::Ifdef), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("aaaa".to_string()), 1)
    );
}

#[test]
fn macro_ifndef() {
    let mut b = "#ifndef aaaa".as_bytes();
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::Macro(Macro::Ifndef), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("aaaa".to_string()), 1)
    );
}

#[test]
fn macro_else() {
    let mut b = "#else".as_bytes();
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::from((Symbol::Sharp, 1)),);
    assert_eq!(l.next(true), Token::from((Symbol::Else, 1)),);
}

#[test]
fn macro_endif() {
    let mut b = "#endif".as_bytes();
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::Macro(Macro::Endif), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn macro_define() {
    let mut b = "#define HELLO (12)".as_bytes();
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::Macro(Macro::Define), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("HELLO".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::LeftParen), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Number("12".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::RightParen), 1)
    );

    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn newline_in_declare_block() {
    let mut b = "declare ok{\n}".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Declare), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("ok".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 2)
    );

    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 2));
}

#[test]
fn next_nl() {
    let mut b = "#define HELLO ok\n declare HELLO{\n}".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::Macro(Macro::Define), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("HELLO".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("ok".to_string()), 1)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Declare), 2));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("HELLO".to_string()), 2)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 2)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 3)
    );
}

#[test]
fn comment_00() {
    let mut b = "declare hello {
            // this is inputs.
            input ok[12];
        }"
        .as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Declare), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("hello".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::OpeningBrace), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(
            TokenClass::CPPStyleComment(" this is inputs.".to_string()),
            2
        )
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Input), 3));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("ok".to_string()), 3)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::LeftSquareBracket), 3)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Number("12".to_string()), 3)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::RightSquareBracket), 3)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 3)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::ClosingBrace), 4)
    );
}

// 2'b00
#[test]
fn number_00() {
    let mut b = "#define SYSTEM_FUNCT_CONTROL    (2'b00)".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::Macro(Macro::Define), 1));
    assert_eq!(
        l.next(true),
        Token::new(
            TokenClass::Identifire("SYSTEM_FUNCT_CONTROL".to_string()),
            1
        )
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::LeftParen), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Number("2'b00".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::RightParen), 1)
    );
}

// 4'hf
#[test]
fn number_01() {
    let mut b = "#define SYSTEM_FUNCT_CONTROL    (4'hf)".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::Macro(Macro::Define), 1));
    assert_eq!(
        l.next(true),
        Token::new(
            TokenClass::Identifire("SYSTEM_FUNCT_CONTROL".to_string()),
            1
        )
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::LeftParen), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Number("4'hf".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::RightParen), 1)
    );
}

// 0b1000
#[test]
fn number_02() {
    let mut b = "#define SYSTEM_FUNCT_CONTROL    (0b1000)".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::Macro(Macro::Define), 1));
    assert_eq!(
        l.next(true),
        Token::new(
            TokenClass::Identifire("SYSTEM_FUNCT_CONTROL".to_string()),
            1
        )
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::LeftParen), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Number("0b1000".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::RightParen), 1)
    );
}

// 0x1000
#[test]
fn number_03() {
    let mut b = "#define SYSTEM_FUNCT_CONTROL    (0x1000)".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::Macro(Macro::Define), 1));
    assert_eq!(
        l.next(true),
        Token::new(
            TokenClass::Identifire("SYSTEM_FUNCT_CONTROL".to_string()),
            1
        )
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::LeftParen), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Number("0x1000".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::RightParen), 1)
    );
}

#[test]
fn define_path() {
    let mut b = "#define MEMORY_HEX \"../hexs/rv32ui-p-xori.hex\"".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Sharp), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::Macro(Macro::Define), 1));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("MEMORY_HEX".to_string()), 1)
    );
    assert_eq!(
        l.next(true),
        Token::new(
            TokenClass::String("../hexs/rv32ui-p-xori.hex".to_string()),
            1
        )
    );
}

#[test]
fn mutiline_comment_00() {
    let mut b = "/**/".as_bytes();
    let mut l = Lexer::new(&mut b);

    let result: Vec<String> = vec![""].iter().map(|s| s.to_string()).collect();

    assert_eq!(l.next(true), Token::new(TokenClass::CStyleComment(result), 1));
}

#[test]
fn mutiline_comment_01() {
    let mut b = "/* hello */".as_bytes();
    let mut l = Lexer::new(&mut b);

    let result: Vec<String> = vec![" hello "].iter().map(|s| s.to_string()).collect();

    assert_eq!(l.next(true), Token::new(TokenClass::CStyleComment(result), 1));
}

#[test]
fn mutiline_comment_02() {
    let mut b = "/*hello\n*/".as_bytes();
    let mut l = Lexer::new(&mut b);

    let result: Vec<String> = vec!["hello", ""].iter().map(|s| s.to_string()).collect();

    assert_eq!(l.next(true), Token::new(TokenClass::CStyleComment(result), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn mutiline_comment_03() {
    let mut b = "/*\ndata lines\n*/".as_bytes();
    let mut l = Lexer::new(&mut b);

    let result: Vec<String> = vec!["", "data lines", ""]
        .iter()
        .map(|s| s.to_string())
        .collect();

    assert_eq!(l.next(true), Token::new(TokenClass::CStyleComment(result), 1));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn module_00() {
    let mut b = "module test {}".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Module, 1)));

    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("test".to_string()), 1))
    );

    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));

    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn module_wire_00() {
    let mut b = "module test { wire ok; }".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Module, 1)));

    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("test".to_string()), 1))
    );

    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::Wire, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("ok".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::Semicolon, 1)));

    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn module_wire_01() {
    let mut b = "module test { wire ok[12]; }".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Module, 1)));

    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("test".to_string()), 1))
    );

    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::Wire, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("ok".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::LeftSquareBracket, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Number("12".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::RightSquareBracket, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::Semicolon, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn reg_00() {
    let mut b = "module test { reg a; }".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Module, 1)));

    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("test".to_string()), 1))
    );

    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));

    assert_eq!(l.next(true), Token::from((Symbol::Reg, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("a".to_string()), 1))
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 1)
    );

    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn func_self_00() {
    let mut b = "module test { wire a, b, c; func_self aa(a, b): c;}".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Module, 1)));

    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("test".to_string()), 1))
    );

    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));

    assert_eq!(l.next(true), Token::from((Symbol::Wire, 1)));

    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("a".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Comma), 1));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("b".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Comma), 1));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("c".to_string()), 1))
    );

    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 1)
    );

    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::FuncSelf), 1)
    );
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("aa".to_string()), 1))
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::LeftParen), 1)
    );
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("a".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Comma), 1));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("b".to_string()), 1))
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::RightParen), 1)
    );
    assert_eq!(l.next(true), Token::new(TokenClass::Symbol(Symbol::Colon), 1));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("c".to_string()), 1))
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 1)
    );
    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));

    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn proc_00() {
    let mut b = "module test { proc_name proc_a(); }".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Module, 1)));

    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("test".to_string()), 1))
    );

    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::ProcName, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("proc_a".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::LeftParen, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::RightParen, 1)));

    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 1)
    );

    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn state_name_00() {
    let mut b = "module test { state_name state1;}".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Module, 1)));

    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("test".to_string()), 1))
    );

    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::StateName, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("state1".to_string()), 1))
    );

    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 1)
    );

    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn mem_00() {
    let mut b = "module test { mem aa[12];}".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Module, 1)));

    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("test".to_string()), 1))
    );

    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::Mem, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("aa".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::LeftSquareBracket, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Number("12".to_string()), 1))
    );

    assert_eq!(l.next(true), Token::from((Symbol::RightSquareBracket, 1)));

    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 1)
    );

    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn wire_assign_00() {
    let mut b = "module test { wire a; a = 1'b1;}".as_bytes();
    let mut l = Lexer::new(&mut b);
    assert_eq!(l.next(true), Token::from((Symbol::Module, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("test".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::Wire, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("a".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::Semicolon, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("a".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::Equal, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Number("1'b1".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::Semicolon, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn plus_00() {
    let mut b = "module test { wire a; a = a + 1'b1;}".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Module, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("test".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::Wire, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("a".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::Semicolon, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("a".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::Equal, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("a".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Operator::Plus, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Number("1'b1".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::Semicolon, 1)));

    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn reg_assignment_00() {
    let mut b = "module test { reg a; a := a + 1'b1;}".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Module, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("test".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::Reg, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("a".to_string()), 1))
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 1)
    );

    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("a".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::RegAssign, 1)),);
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("a".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Operator::Plus, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Number("1'b1".to_string()), 1))
    );
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Symbol(Symbol::Semicolon), 1)
    );
    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
    assert_eq!(l.next(true), Token::new(TokenClass::EndOfProgram, 1));
}

#[test]
fn func_block_00() {
    let mut b = "func test {}".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Func, 1)));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("test".to_string()), 1)
    );
    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
}

#[test]
fn func_block_return() {
    let mut b = "func test { return mtvec; }".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Func, 1)));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Identifire("test".to_string()), 1)
    );
    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::Return, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("mtvec".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::Semicolon, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
}

#[test]
fn any_block_00() {
    let mut b = "any {}".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Any, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
}

#[test]
fn or_00() {
    let mut b = "return a | b;".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Return, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("a".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Operator::Pipe, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("b".to_string()), 1))
    );
}

#[test]
fn any_else_00() {
    let mut b = "any { a: {} else: {} }".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::Any, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("a".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::Colon, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::Else, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::Colon, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
}

#[test]
fn gt_lt_00() {
    let mut b = "address >= 12'h3a0 && address <= 12'h3bf".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("address".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Operator::GreaterEq, 1)));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Number("12'h3a0".to_string()), 1)
    );
    assert_eq!(l.next(true), Token::from((Operator::LogicAnd, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("address".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Operator::LessEq, 1)));
    assert_eq!(
        l.next(true),
        Token::new(TokenClass::Number("12'h3bf".to_string()), 1)
    );
}

#[test]
fn state_00() {
    let mut b = "state idle {}".as_bytes();
    let mut l = Lexer::new(&mut b);

    assert_eq!(l.next(true), Token::from((Symbol::State, 1)));
    assert_eq!(
        l.next(true),
        Token::from((TokenClass::Identifire("idle".to_string()), 1))
    );
    assert_eq!(l.next(true), Token::from((Symbol::OpeningBrace, 1)));
    assert_eq!(l.next(true), Token::from((Symbol::ClosingBrace, 1)));
}
