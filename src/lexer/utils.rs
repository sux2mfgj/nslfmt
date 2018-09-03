use std::iter::Peekable;

use lexer::analyzer::LexItem;

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
    use lexer::analyzer::create_lex_item_word;

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
