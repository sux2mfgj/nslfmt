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
    Semicolon,
    SquareBracketL,
    SquareBracketR,
    Number(i64),
    Equal,
    Other,
}

fn lexical_analyzer(input: &String) -> Result<Vec<LexItem>, String>
{
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            'a' ... 'z' | 'A' ... 'Z' | '_' => {
                let n = get_lexical_type(&mut it);
                result.push(n);
            }
            '0' ... '9' => {
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
        if c_next.is_alphanumeric() | (c_next == '_') {
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

fn get_number<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> LexItem
{
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
mod lex_type {
    use super::*;

    #[test]
    fn module() {
        let s = "module".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Module, get_lexical_type(&mut it));
    }

    #[test]
    fn module_space() {
        let s = "module ".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Module, get_lexical_type(&mut it));
    }

    #[test]
    fn declare() {
        let s = "declare".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Declare, get_lexical_type(&mut it));
    }
    #[test]
    fn declare_parenthesis() {
        let s = "declare {}".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Declare, get_lexical_type(&mut it));
    }

    #[test]
    fn other() {
        let s = "aaa".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Other, get_lexical_type(&mut it));
    }

    #[test]
    fn other_newline() {
        let s = "aa\n{}".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Other, get_lexical_type(&mut it));
    }
}
#[cfg(test)]
mod lexical_analyzer {
    use super::*;

    #[test]
    fn module_parent() {
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
    fn declare_parent() {
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
    fn other_other() {
        let result = lexical_analyzer(
            &String::from(
                "reg rxd"
                ));
        assert_eq!(result.unwrap(),
                   [LexItem::Other,
                    LexItem::Other]);
    }

    #[test]
    fn internal_underscore() {
        let result = lexical_analyzer(
            &String::from(
                "reg rx_d"
                ));
        assert_eq!(result.unwrap(),
                   [LexItem::Other,
                    LexItem::Other]);
    }

    #[test]
    fn head_underscore() {
        let result = lexical_analyzer(
            &String::from(
                "reg _rx_d"
                ));
        assert_eq!(result.unwrap(),
                   [LexItem::Other,
                    LexItem::Other]);
    }

    #[test]
    fn semicolon() {
        let result = lexical_analyzer(
            &String::from(
                "reg _rx_d;"
                ));
        assert_eq!(result.unwrap(),
                   [LexItem::Other,
                    LexItem::Other,
                    LexItem::Semicolon]);
    }

    #[test]
    fn square_bracket() {
        let result = lexical_analyzer(
            &String::from(
                "reg _rx_d[0];"
                ));
        assert_eq!(result.unwrap(),
                   [LexItem::Other,
                    LexItem::Other,
                    LexItem::SquareBracketL,
                    LexItem::Number(0),
                    LexItem::SquareBracketR,
                    LexItem::Semicolon]);
    }

    #[test]
    fn mod_reg_index() {
        let result = lexical_analyzer(
            &String::from(
                "module {
                    reg test_ok[12];
                }"
                ));
        assert_eq!(result.unwrap(),
                   [LexItem::Module,
                    LexItem::CurlyBracketL,
                    LexItem::Other,
                    LexItem::Other,
                    LexItem::SquareBracketL,
                    LexItem::Number(12),
                    LexItem::SquareBracketR,
                    LexItem::Semicolon,
                    LexItem::CurlyBracketR]);
    }

    #[test]
    fn equal() {
        let result = lexical_analyzer(
            &String::from(
                "module
                {
                    reg counter[10] = 0;
                }"));
        assert_eq!(result.unwrap(),
                   [LexItem::Module,
                    LexItem::CurlyBracketL,
                    LexItem::Other,
                    LexItem::Other,
                    LexItem::SquareBracketL,
                    LexItem::Number(10),
                    LexItem::SquareBracketR,
                    LexItem::Equal,
                    LexItem::Number(0),
                    LexItem::Semicolon,
                    LexItem::CurlyBracketR,
                   ] )
    }
}

fn main() {
}

