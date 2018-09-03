use std::iter::Peekable;

#[derive(Debug)]
enum GrammerItem {
    Dec,
    Mod,
    Paren
}

#[derive(Debug)]
struct ParseNode {
    children: Vec<ParseNode>,
    entry: GrammerItem,
}

impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: GrammerItem::Paren,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum LexItem{
    Module,
    Declare,
    CurlyBracketL,
    CurlyBracketR,
    Other,
}

fn lexical_analyzer(input: &String) -> Result<Vec<LexItem>, String>
{
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            'a' ... 'z' | 'A' ... 'Z' => {
                let n = get_lexical_type(&mut it);
                result.push(n);
            }
            ' ' | '\t' | '\n' => {
                it.next();
            }
            '{' => {
                it.next();
                result.push(LexItem::CurlyBracketL);
            }
            '}' => {
                it.next();
                result.push(LexItem::CurlyBracketR);
            }
            _ => {
                return Err(format!("unexpected character {}", c))
            }
        }
    }
    Ok(result)
}

fn get_lexical_type<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> LexItem
{

    let mut word = String::new();
    while let Some(&c_next) = iter.peek() {
        if c_next.is_alphanumeric() {
            word.push_str(&c_next.to_string());
            iter.next();
        }
        else {
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
        _ => {
            return LexItem::Other;
        }
    }
}

#[cfg(test)]
mod lex_tests {
    use super::*;

    #[test]
    fn lex_test_01() {
        let s = "module".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Module, get_lexical_type(&mut it));
    }

    #[test]
    fn lex_test_02() {
        let s = "module ".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Module, get_lexical_type(&mut it));
    }

    #[test]
    fn lex_test_03() {
        let s = "declare".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Declare, get_lexical_type(&mut it));
    }
    #[test]
    fn lex_test_04() {
        let s = "declare {}".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Declare, get_lexical_type(&mut it));
    }

    #[test]
    fn lex_test_05() {
        let s = "aaa".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Other, get_lexical_type(&mut it));
    }

    #[test]
    fn lex_test_06() {
        let s = "aa\n{}".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Other, get_lexical_type(&mut it));
    }

    #[test]
    fn la_test_01() {
        let result = lexical_analyzer(
            &String::from(
                "module
                {
                }
                "
                ));
        assert_eq!(result.unwrap(),
                   [LexItem::Module,
                    LexItem::CurlyBracketL,
                    LexItem::CurlyBracketR]);
    }
    #[test]
    fn la_test_02() {
        let result = lexical_analyzer(
            &String::from(
                "declare {}"
                ));
        assert_eq!(result.unwrap(),
                   [LexItem::Declare,
                    LexItem::CurlyBracketL,
                    LexItem::CurlyBracketR]);
    }
    #[test]
    fn la_test_03() {
        let result = lexical_analyzer(
            &String::from(
                "reg rxd"
                //"reg rx_d"
                ));
        assert_eq!(result.unwrap(),
                   [LexItem::Other,
                    LexItem::Other]);
    }
}

fn main() {
}

