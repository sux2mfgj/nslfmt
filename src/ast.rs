pub enum ASTClass {
    Declare(String, Vec<ASTNode>),
    FuncIn(String, String, String),
    FuncOut(String, String, String),
    Input(String, usize),
    Output(String, usize),
    InOut(String, usize),
}

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
