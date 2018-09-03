use lexer::utils::{get_number, get_word};

#[derive(Debug, Clone, PartialEq)]
pub enum LexItem {
    Module,
    Declare,
    CurlyBracketL,
    CurlyBracketR,
    Semicolon,
    SquareBracketL,
    SquareBracketR,
    Number(i64),
    Equal,
    Word(String),
    Input,
    Output,
    InOut,
}

pub fn lexical_analyzer(input: &String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            'a'...'z' | 'A'...'Z' | '_' => {
                let n = get_word(&mut it);
                result.push(n);
            }
            '0'...'9' => {
                let n = get_number(&mut it);
                result.push(n);
            }
            ' ' | '\t' | '\n' => {
                it.next();
            }
            '=' => {
                it.next();
                result.push(LexItem::Equal);
            }
            '{' => {
                it.next();
                result.push(LexItem::CurlyBracketL);
            }
            '}' => {
                it.next();
                result.push(LexItem::CurlyBracketR);
            }
            '[' => {
                it.next();
                result.push(LexItem::SquareBracketL);
            }
            ']' => {
                it.next();
                result.push(LexItem::SquareBracketR);
            }
            ';' => {
                it.next();
                result.push(LexItem::Semicolon);
            }
            _ => return Err(format!("unexpected character {}", c)),
        }
    }
    Ok(result)
}

#[cfg(test)]
pub fn create_lex_item_word(x: &str) -> LexItem {
    LexItem::Word(x.to_string())
}

#[cfg(test)]
mod lexical_analyzer {
    use super::*;

    #[test]
    fn module_parent() {
        let result = lexical_analyzer(&String::from(
            "module
                {
                }
                ",
        ));
        assert_eq!(
            result.unwrap(),
            [
                LexItem::Module,
                LexItem::CurlyBracketL,
                LexItem::CurlyBracketR
            ]
        );
    }

    #[test]
    fn declare_parent() {
        let result = lexical_analyzer(&String::from("declare {}"));
        assert_eq!(
            result.unwrap(),
            [
                LexItem::Declare,
                LexItem::CurlyBracketL,
                LexItem::CurlyBracketR
            ]
        );
    }

    #[test]
    fn other_other() {
        let result = lexical_analyzer(&String::from("reg rxd"));
        assert_eq!(
            result.unwrap(),
            [create_lex_item_word("reg"), create_lex_item_word("rxd")]
        );
    }

    #[test]
    fn internal_underscore() {
        let result = lexical_analyzer(&String::from("reg rx_d"));
        assert_eq!(
            result.unwrap(),
            [create_lex_item_word("reg"), create_lex_item_word("rx_d")]
        );
    }

    #[test]
    fn head_underscore() {
        let result = lexical_analyzer(&String::from("reg _rx_d"));
        assert_eq!(
            result.unwrap(),
            [create_lex_item_word("reg"), create_lex_item_word("_rx_d")]
        );
    }

    #[test]
    fn semicolon() {
        let result = lexical_analyzer(&String::from("reg _rx_d;"));
        assert_eq!(
            result.unwrap(),
            [
                create_lex_item_word("reg"),
                create_lex_item_word("_rx_d"),
                LexItem::Semicolon
            ]
        );
    }

    #[test]
    fn square_bracket() {
        let result = lexical_analyzer(&String::from("reg _rx_d[0];"));
        assert_eq!(
            result.unwrap(),
            [
                create_lex_item_word("reg"),
                create_lex_item_word("_rx_d"),
                LexItem::SquareBracketL,
                LexItem::Number(0),
                LexItem::SquareBracketR,
                LexItem::Semicolon
            ]
        );
    }

    #[test]
    fn mod_reg_index() {
        let result = lexical_analyzer(&String::from(
            "module {
                    reg test_ok[12];
                }",
        ));
        assert_eq!(
            result.unwrap(),
            [
                LexItem::Module,
                LexItem::CurlyBracketL,
                create_lex_item_word("reg"),
                create_lex_item_word("test_ok"),
                LexItem::SquareBracketL,
                LexItem::Number(12),
                LexItem::SquareBracketR,
                LexItem::Semicolon,
                LexItem::CurlyBracketR
            ]
        );
    }

    #[test]
    fn equal() {
        let result = lexical_analyzer(&String::from(
            "module
                {
                    reg counter[10] = 0;
                }",
        ));
        assert_eq!(
            result.unwrap(),
            [
                LexItem::Module,
                LexItem::CurlyBracketL,
                create_lex_item_word("reg"),
                create_lex_item_word("counter"),
                LexItem::SquareBracketL,
                LexItem::Number(10),
                LexItem::SquareBracketR,
                LexItem::Equal,
                LexItem::Number(0),
                LexItem::Semicolon,
                LexItem::CurlyBracketR,
            ]
        )
    }

    #[test]
    fn in_out_inout() {
        let result = lexical_analyzer(&String::from(
            "declare
                {
                    output scl;
                    inout sda;
                    input address[8];
                }",
        ));
        assert_eq!(
            result.unwrap(),
            [
                LexItem::Declare,
                LexItem::CurlyBracketL,
                LexItem::Output,
                create_lex_item_word("scl"),
                LexItem::Semicolon,
                LexItem::InOut,
                create_lex_item_word("sda"),
                LexItem::Semicolon,
                LexItem::Input,
                create_lex_item_word("address"),
                LexItem::SquareBracketL,
                LexItem::Number(8),
                LexItem::SquareBracketR,
                LexItem::Semicolon,
                LexItem::CurlyBracketR,
            ]
        )
    }
}
