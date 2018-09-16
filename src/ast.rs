#[derive(Debug, Clone, PartialEq)]
pub enum ASTClass {
    Declare(String, Vec<ASTNode>, Vec<ASTNode>),
    FuncIn(String, Vec<String>, String),
    FuncOut(String, Vec<String>, String),
    Input(String, String),
    Output(String, String),
    InOut(String, String),
    MacroInclude(String),
    MacroDefine(String, String),
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
