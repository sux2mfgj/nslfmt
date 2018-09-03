use std::iter::Peekable;

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

pub fn get_word<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> LexItem {
    let mut word = String::new();
    while let Some(&c_next) = iter.peek() {
        if c_next.is_alphanumeric() | (c_next == '_') {
            word.push_str(&c_next.to_string());
            iter.next();
        } else {
            break;
        }
    }
    match word.as_str() {
        "declare" => {
            return LexItem::Declare;
        }
        "module" => {
            return LexItem::Module;
        }
        "input" => {
            return LexItem::Input;
        }
        "output" => {
            return LexItem::Output;
        }
        "inout" => {
            return LexItem::InOut;
        }
        _ => {
            return LexItem::Word(word);
        }
    }
}

#[cfg(test)]
mod get_word {
    use super::*;

    #[test]
    fn module() {
        let s = "module".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Module, get_word(&mut it));
    }

    #[test]
    fn module_space() {
        let s = "module ".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Module, get_word(&mut it));
    }

    #[test]
    fn declare() {
        let s = "declare".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Declare, get_word(&mut it));
    }
    #[test]
    fn declare_parenthesis() {
        let s = "declare {}".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Declare, get_word(&mut it));
    }

    #[test]
    fn other() {
        let s = "aaa".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(create_lex_item_word("aaa"), get_word(&mut it));
    }

    #[test]
    fn other_newline() {
        let s = "aa\n{}".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(create_lex_item_word("aa"), get_word(&mut it));
    }

    #[test]
    fn input() {
        let s = "input".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Input, get_word(&mut it));
    }

    #[test]
    fn output() {
        let s = "output".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Output, get_word(&mut it));
    }

    #[test]
    fn inout() {
        let s = "inout".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::InOut, get_word(&mut it));
    }
}

pub fn get_number<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> LexItem {
    let mut number = 0;
    /*
     * === TODO ===
     * implement other number representations.
     * e.g. 0x10, 10'h12, 0x1000_1000, 4'b11, 0b11, 0o24 etc...
     * you have to see a language reference of NSL.
     */
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<i64>()) {
        number = number * 10 + digit;
        iter.next();
    }
    LexItem::Number(number)
}

#[cfg(test)]
mod get_number {
    use super::*;

    #[test]
    fn digit() {
        let s = "10".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Number(10), get_number(&mut it));
    }

    /*
    #[test]
	fn hex() {
        let s = "0x10".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Number(0x10), get_number(&mut it));
    }
    */
}
