#[derive(Debug, Clone, PartialEq)]
pub enum ASTClass {
    Declare(String, Vec<ASTNode>, Vec<ASTNode>),
    FuncIn(String, Vec<String>, String),
    FuncOut(String, Vec<String>, String),
    Input(String, String),
    Output(String, String),
    InOut(String, String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASTNode {
    pub class: ASTClass,
}

impl ASTNode {
    pub fn new(class: ASTClass) -> ASTNode {
        ASTNode{
            class: class,
        }
    }
}

//pub struct FuncNode {
//    pub class: ASTClass,
//    pub name: String,
//    pub in_port: String,
//    pub out_port: String,
//}
//
//pub struct DeclareNode<'a> {
//    pub class: ASTClass,
//    pub name: String,
//    pub io  : &'a Vec<IONode>,
//    pub func: &'a Vec<FuncNode>,
//}
