use token::Operator;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTClass {
    Identifire(String),
    Number(String),
    String(String),
    /*
     *  block
     *  e.g.
     *      {
     *          input hello[12];
     *          func_out ok() : hello;
     *      }
     */
    Block(Vec<Box<ASTNode>>),
    WidthBlock(Box<ASTNode>),
    Operator(Operator),

    // identifire, block
    Declare(Box<ASTNode>, Box<ASTNode>),

    // identifire, inputs, output
    FuncIn(Box<ASTNode>, Vec<Box<ASTNode>>, Box<ASTNode>),

    // identifire, outputs, input
    FuncOut(Box<ASTNode>, Vec<Box<ASTNode>>, Box<ASTNode>),
    /*
     *  identifire, expression or Identifire
     *  e.g.
     *      input hello[A_WIDTH / 2];
     *      input hello[B_WIDTH];
     *      input hello[3];
     */
    Input(Box<ASTNode>, Box<ASTNode>),
    Output(Box<ASTNode>, Box<ASTNode>),
    InOut(Box<ASTNode>, Box<ASTNode>),
    MacroInclude(Box<ASTNode>),
    MacroUndef(Box<ASTNode>),
    MacroIfdef(Box<ASTNode>),
    MacroIfndef(Box<ASTNode>),
    MacroElse,
    MacroEndif,
    MacroDefine(Box<ASTNode>, String),
    //          operand     , operation   , operand
    Expression(Box<ASTNode>, Operator, Box<ASTNode>),
    EndOfProgram,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTNode {
    pub class: ASTClass,
}

impl ASTNode {
    pub fn new(class: ASTClass) -> ASTNode {
        ASTNode { class: class }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.class {
            ASTClass::Declare(ref id, ref interfaces) => {
                return write!(f, "declare {} {}", id, interfaces);
            }
            ASTClass::Identifire(ref s) => {
                return write!(f, "{}", s);
            }
            ASTClass::Block(ref list) => {
                let mut list_str = String::new();
                for node in list {
                    list_str.push_str(&format!("{}\n", node));
                }

                return write!(f, "\n{{\n{}}}", list_str);
            }
            _ => {
                panic!("For the node {:?}, fmt::Display does not implemented yet.",
                       self);
            }
        }
    }
}

