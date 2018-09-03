mod lexer;

use lexer::LexItem;
//use std::iter::Peekable;

#[derive(Debug)]
enum GrammerItem {
    Top,
    Declare,
    //Module,
    Word(String),
    //If
    //Proc
    //Func
    Paren,
}

#[derive(Debug)]
struct ParseNode {
    children: Vec<ParseNode>,
    entry: GrammerItem,
}

impl ParseNode {
    pub fn new(item: GrammerItem) -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: item,
        }
    }
}

fn parse_declare(tokens: &Vec<LexItem>, mut pos: usize) -> Result<(ParseNode, usize), String> {
    let mut dec = ParseNode::new(GrammerItem::Declare);

    match tokens.get(pos) {
        Some(&LexItem::Word(ref word)) => {
            pos = pos + 1;
            dec.children
                .push(ParseNode::new(GrammerItem::Word(word.to_string())));
        }
        _ => {
            return Err(format!("unexpected word"));
        }
    }

    match tokens.get(pos) {
        Some(&LexItem::CurlyBracketL) => {
            pos = pos + 1;
            dec.children.push(ParseNode::new(GrammerItem::Paren));
        }
        _ => {
            return Err(format!("unexpected word"));
        }
    }

    loop {
        match tokens.get(pos) {
            Some(&LexItem::CurlyBracketR) => {
                dec.children.push(ParseNode::new(GrammerItem::Paren));
                break;
            }
            _ => {
                //TODO
            }
        }
    }

    //let t_cb_left = tokens.get(p);
    Ok((dec, pos))
}

fn parse_expression(tokens: &Vec<LexItem>, mut pos: usize) -> Result<(ParseNode, usize), String> {
    let mut top = ParseNode::new(GrammerItem::Top);
    loop {
        let t = tokens.get(pos);
        pos = pos + 1;
        if pos >= tokens.len() {
            return Ok::<(ParseNode, usize), String>((top, pos));
        }
        match t {
            Some(&LexItem::Declare) => {
                //let mut dec = ParseNode::new(GrammerItem::Declare);
                let (d_node, n_pos) = try!(parse_declare(tokens, pos));
                pos = n_pos;
                top.children.push(d_node);
            }
            // TODO
            // Some(&LexItem::Module) => {}
            _ => {
                return Err(format!(
                    "exptect the declare or module, but received {:?}",
                    t
                ));
            }
        }
    }
}

fn parse(input: &String) -> Result<ParseNode, String> {
    let tokens = try!(lexer::lexical_analyzer(input));
    parse_expression(&tokens, 0).and_then(|(n, i)| {
        if i == tokens.len() {
            Ok(n)
        } else {
            Err(format!(
                "token was left. next token is {:?} at {}",
                tokens[i], i
            ))
        }
    })
}

fn main() {
    println!("{:?}", parse(&String::from("declare hello {}")));
}
