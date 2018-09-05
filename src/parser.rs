use lexer::*;

#[derive(Debug, PartialEq)]
pub enum GrammerItem {
    Top,
    Declare,
    //Module,
    Word(String),
    //If
    //Proc
    //Func
    Paren,
    Interface,
}

#[derive(Debug, PartialEq)]
pub struct ParseNode {
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

fn parse_interface(tokens: &Vec<LexItem>, mut pos: usize) -> Result<(ParseNode, usize), String> {

    let t = tokens.get(pos).unwrap();
    if t != LexItem::Input & t != Some(LexItem::Output) & t != Some(LexItem::InOut)
    {
        return Err(format!("unexpected word"));
    }
    let mut interface = ParseNode::new(t);
    /*
    match tokens.get(pos)
    {
        Some(&LexItem::Input) | Some(&LexItem::InOut) | Some(&LexItem::Output)=> {

        }
        _ => {
            return Err(format!("unexpected word"));
        }
    }
    */
    Err(format!("unexpected word"))
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
        pos = pos + 1;
    }

    //let t_cb_left = tokens.get(p);
    Ok((dec, pos))
}

#[cfg(test)]
mod parse_declare {
    use super::*;

    #[test]
    fn hello() {
        let tokens = lexical_analyzer(&String::from("hello {}")).unwrap();
        let (node, _) = parse_declare(&tokens, 0).unwrap();

        let mut result = ParseNode::new(GrammerItem::Declare);
        result
            .children
            .push(ParseNode::new(GrammerItem::Word(String::from("hello"))));
        result.children.push(ParseNode::new(GrammerItem::Paren));
        result.children.push(ParseNode::new(GrammerItem::Paren));

        assert_eq!(node, result);
    }

    #[test]
    fn goodmorning_newline() {
        let tokens = lexical_analyzer(&String::from(
            "goodmorning
                                                    {}",
        )).unwrap();
        let (node, _) = parse_declare(&tokens, 0).unwrap();

        let mut result = ParseNode::new(GrammerItem::Declare);
        result
            .children
            .push(ParseNode::new(GrammerItem::Word(String::from(
                "goodmorning",
            ))));
        result.children.push(ParseNode::new(GrammerItem::Paren));
        result.children.push(ParseNode::new(GrammerItem::Paren));

        assert_eq!(node, result);
    }

    #[test]
    fn reg() {
        let tokens = lexical_analyzer(&String::from(
            "helo
                                                    {
                                                        input a[2];
                                                        }",
        )).unwrap();
        let (node, _) = parse_declare(&tokens, 0).unwrap();

        let mut result = ParseNode::new(GrammerItem::Declare);
        result
            .children
            .push(ParseNode::new(GrammerItem::Word(String::from("helo"))));
        result.children.push(ParseNode::new(GrammerItem::Paren));
        result.children.push(ParseNode::new(GrammerItem::Paren));

        assert_eq!(node, result);
    }
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

#[cfg(test)]
mod parse_expression {
    use super::*;

    #[test]
    fn declare() {
        let tokens = lexical_analyzer(&String::from("declare hello {}")).unwrap();
        let (node, _) = parse_expression(&tokens, 0).unwrap();

        let mut result = ParseNode::new(GrammerItem::Top);
        let mut dec = ParseNode::new(GrammerItem::Declare);
        dec.children
            .push(ParseNode::new(GrammerItem::Word(String::from("hello"))));
        dec.children.push(ParseNode::new(GrammerItem::Paren));
        dec.children.push(ParseNode::new(GrammerItem::Paren));
        result.children.push(dec);

        assert_eq!(node, result);
    }

    #[test]
    fn declare_newline() {
        let tokens = lexical_analyzer(&String::from(
            "declare hello
        {

            }",
        )).unwrap();
        let (node, _) = parse_expression(&tokens, 0).unwrap();

        let mut result = ParseNode::new(GrammerItem::Top);
        let mut dec = ParseNode::new(GrammerItem::Declare);
        dec.children
            .push(ParseNode::new(GrammerItem::Word(String::from("hello"))));
        dec.children.push(ParseNode::new(GrammerItem::Paren));
        dec.children.push(ParseNode::new(GrammerItem::Paren));
        result.children.push(dec);

        assert_eq!(node, result);
    }
}

pub fn parse(input: &String) -> Result<ParseNode, String> {
    let tokens = try!(lexical_analyzer(input));
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

#[cfg(test)]
mod parse {
    use super::*;

    #[test]
    fn hello() {
        let tokens = lexical_analyzer(&String::from("declare hello {}")).unwrap();
        let (node, _) = parse_expression(&tokens, 0).unwrap();

        let mut result = ParseNode::new(GrammerItem::Top);
        let mut dec = ParseNode::new(GrammerItem::Declare);
        dec.children
            .push(ParseNode::new(GrammerItem::Word(String::from("hello"))));
        dec.children.push(ParseNode::new(GrammerItem::Paren));
        dec.children.push(ParseNode::new(GrammerItem::Paren));
        result.children.push(dec);

        assert_eq!(node, result);
    }

    #[test]
    fn hello_interface() {
        let tokens = lexical_analyzer(&String::from("declare hello {output a}")).unwrap();
        let (node, _) = parse_expression(&tokens, 0).unwrap();

        let mut result = ParseNode::new(GrammerItem::Top);
        let mut dec = ParseNode::new(GrammerItem::Declare);
        dec.children
            .push(ParseNode::new(GrammerItem::Word(String::from("hello"))));
        dec.children.push(ParseNode::new(GrammerItem::Paren));
        dec.children.push(ParseNode::new(GrammerItem::Paren));
        result.children.push(dec);
        assert_eq!(node, result);
    }
}
