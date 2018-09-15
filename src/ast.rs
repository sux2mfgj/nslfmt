#[derive(Debug, Clone, PartialEq)]
pub enum ASTClass {
    Declare(String, Vec<ASTNode>, Vec<ASTNode>),
    FuncIn(String, Vec<String>, String),
    FuncOut(String, Vec<String>, String),
    Input(String, String),
    Output(String, String),
    InOut(String, String),
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
