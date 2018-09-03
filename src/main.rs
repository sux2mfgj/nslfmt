mod lexer;

use lexer::LexItem;
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

fn main() {
}
