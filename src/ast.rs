#[derive(Debug, Clone, PartialEq)]
pub enum ASTClass {
//    Identifire(String),
//    Number(String),
    // identifire, interfaces(input, output, inout, funcin, funcout)
    //Declare(Box<ASTNode>, Vec<ASTNode>),
    Declare(String, Vec<ASTNode>),
    // identifire, inputs, output
    //FuncIn(Box<ASTNode>, Vec<ASTNode>, Box<ASTNode>),
    FuncIn(String, Vec<String>, String),
    // identifire, outputs, input
    //FuncOut(Box<ASTNode>, Vec<ASTNode>, Box<ASTNode>),
    FuncOut(String, Vec<String>, String),
    //Input(Box<ASTNode>, Box<ASTNode>),
    Input(String, String),
    Output(String, String),
    InOut(String, String),
    MacroInclude(String),
    MacroDefine(String, Vec<ASTNode>),
    MacroUndef(String),
    MacroIfdef(String),
    MacroIfndef(String),
    MacroElse,
    MacroEndif,
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
