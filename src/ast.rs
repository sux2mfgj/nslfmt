use token::Operator;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTClass {
    Identifire(String),
    Number(String),
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
    //FuncIn(Box<ASTNode>, Vec<ASTNode>, Box<ASTNode>),
    FuncIn(String, Vec<String>, String),
    // identifire, outputs, input
    //FuncOut(Box<ASTNode>, Vec<ASTNode>, Box<ASTNode>),
    FuncOut(String, Vec<String>, String),
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
    MacroInclude(String),
    MacroDefine(String, Vec<Box<ASTNode>>),
    MacroUndef(String),
    MacroIfdef(String),
    MacroIfndef(String),
    MacroElse,
    MacroEndif,
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
