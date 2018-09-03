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

fn lex(input: &String) -> Result<Vec<LexItem>, String>
{
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            'a' ... 'z' | 'A' ... 'Z' => {
                let n = get_lex(&mut it);
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

fn get_lex<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> LexItem
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
mod tests {
    use super::*;

    #[test]
    fn lex_test_01() {
        let s = "module".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Module, get_lex(&mut it));
    }

    #[test]
    fn lex_test_02() {
        let s = "module ".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Module, get_lex(&mut it));
    }

    #[test]
    fn lex_test_03() {
        let s = "declare".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Declare, get_lex(&mut it));
    }
    #[test]
    fn lex_test_04() {
        let s = "declare {}".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Declare, get_lex(&mut it));
    }

    #[test]
    fn lex_test_05() {
        let s = "aaa".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Other, get_lex(&mut it));
    }

    #[test]
    fn lex_test_06() {
        let s = "aa\n{}".to_string();
        let mut it = s.chars().peekable();
        assert_eq!(LexItem::Other, get_lex(&mut it));
    }
}

fn main() {
    let result = lex(&String::from("module \n{}"));
    for val in result.unwrap().iter() {
        println!("{:?}", val);
    }
}

