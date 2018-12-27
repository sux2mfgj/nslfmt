use std::collections::LinkedList;
use std::fmt;
use token;

macro_rules! not_implemented {
    () => {
        panic!("not implemented yet. at line {} in {}.", line!(), file!())
    };
}

macro_rules! get_top {
    ($t:ident) => {
        $t.generate().pop_front().unwrap()
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTClass {
    Identifire(String),
    Number(String),
    String(String),
    BitSlice(Box<ASTNode>, Option<Box<ASTNode>>),
    Operator(token::Operator),
    UnaryOperator(token::UnaryOperator),
    /*
     *  block
     *  e.g.
     *      {
     *          input hello[12];
     *          func_out ok() : hello;
     *      }
     */
    Block(Vec<Box<ASTNode>>),
    // ----- Declare ------
    // identifire, block
    Declare(Box<ASTNode>, Box<ASTNode>),

    // identifire, inputs, output
    FuncIn(Box<ASTNode>, Vec<Box<ASTNode>>, Option<Box<ASTNode>>),
    // identifire, outputs, input
    FuncOut(Box<ASTNode>, Vec<Box<ASTNode>>, Option<Box<ASTNode>>),
    // identifire, inputs, output
    FuncSelf(
        Box<ASTNode>,
        Vec<Box<ASTNode>>,
        Option<Box<ASTNode>>,
    ),

    /*
     *  identifire, expression or Identifire
     *  e.g.
     *      input hello[A_WIDTH / 2];
     *      input hello[B_WIDTH];
     *      input hello[3];
     */
    Input(Box<ASTNode>, Option<Box<ASTNode>>),
    Output(Box<ASTNode>, Option<Box<ASTNode>>),
    InOut(Box<ASTNode>, Option<Box<ASTNode>>),

    // ----- Module ------
    // identifire, block
    Module(Box<ASTNode>, Box<ASTNode>),
    //      <id(submodule name)>,  { <id> [<expr>] }* ;
    //      e.g.
    //          test in1, in2[2], in3;
    Submodule(Box<ASTNode>, Vec<(Box<ASTNode>, Option<Box<ASTNode>>)>),
    //MacroSubModule(Vec<token::Token>),
    //      id,         , args
    ProcName(Box<ASTNode>, Vec<Box<ASTNode>>),
    StateName(Vec<Box<ASTNode>>),
    //  id          ,[12]        , [12]                 , initial value
    Mem(
        Vec<(
            Box<ASTNode>,
            Box<ASTNode>,
            Option<Box<ASTNode>>,
            Option<Vec<Box<ASTNode>>>,
        )>,
    ),
    //     id          , expression
    Assign(Box<ASTNode>, Box<ASTNode>),
    RegAssign(Box<ASTNode>, Box<ASTNode>),
    //   id          , block
    Func(Box<ASTNode>, Option<Box<ASTNode>>, Box<ASTNode>),
    //  expression       , block
    Any(Vec<(Box<ASTNode>, Box<ASTNode>)>),
    Return(Box<ASTNode>),
    Goto(Box<ASTNode>),
    Else,
    //          <id(submodule)>, <id(port)>
    ModulePort(Box<ASTNode>, Box<ASTNode>),
    FuncCall(Box<ASTNode>, Vec<Box<ASTNode>>, Option<Box<ASTNode>>),
    //  state name, block
    State(Box<ASTNode>, Box<ASTNode>),
    // if (<expression>) <block>, <else_node>
    If(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>),

    // ----- Macros ------
    MacroInclude(Box<ASTNode>),
    MacroUndef(Box<ASTNode>),
    MacroIfdef(Box<ASTNode>),
    MacroIfndef(Box<ASTNode>),
    MacroElse,
    MacroEndif,
    MacroDefine(Box<ASTNode>, Option<String>),

    // wire enable, data[12];
    //              id    , width
    Wire(Vec<(Box<ASTNode>, Option<Box<ASTNode>>)>),
    //          id       , width       , initial_value
    Reg(Vec<(Box<ASTNode>, Option<Box<ASTNode>>, Option<Box<ASTNode>>)>),

    //          operand     , operation   , operand,   is required parances
    Expression(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>),
    //              expr        , bitslice
    BitslicedExpr(Box<ASTNode>, Box<ASTNode>),
    //          unary operator, expression
    UnaryOperation(Box<ASTNode>, Box<ASTNode>),
    CPPStyleComment(String),
    CStyleComment(Vec<String>),
    //     Newline,
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

    pub fn generate(&self) -> LinkedList<String> {
        let mut list = LinkedList::new();
        match self.class {
            ASTClass::Declare(ref id, ref block) => {
                list.push_back(format!("declare {}", id));
                list.append(&mut block.generate());
            }
            ASTClass::Module(ref id, ref block) => {
                list.push_back(format!("module {}", id));
                list.append(&mut block.generate());
            }
            ASTClass::Block(ref contents) => {
                for c in contents {
                    match c.class {
                        ASTClass::Any(_) => {
                            list.append(&mut c.generate());
                        }
                        ASTClass::Func(_, _, _) => {
                            list.append(&mut c.generate());
                        }
                        ASTClass::If(_, _, _) => {
                            list.append(&mut c.generate());
                        }
                        ASTClass::State(_, _) => {
                            list.append(&mut c.generate());
                        }
                        ASTClass::CPPStyleComment(_) => {
                            list.append(&mut c.generate());
                        }
                        //TODO
                        _ => {
                            list.push_back(format!("{};", get_top!(c)));
                        }
                    }
                }
                let mut nm: LinkedList<String> =
                    list.iter().map(|c| format!("    {}", c)).collect();
                nm.push_front("{".to_string());
                nm.push_back("}".to_string());
                return nm;
            }
            ASTClass::Any(ref contents) => {
                for (expr, block) in contents {
                    let expr_str = if let Some(top) = expr.generate().pop_front() {
                        top
                    } else {
                        panic!();
                    };
                    list.push_back(format!("{}:", expr_str));
                    list.append(&mut block.generate());
                }
                let mut nm: LinkedList<String> =
                    list.iter().map(|c| "    ".to_string() + c).collect();
                nm.push_front("{".to_string());
                nm.push_back("}".to_string());
                nm.push_front("any".to_string());
                return nm;
            }
            ASTClass::Else => {
                list.push_back("else".to_string());
            }
            ASTClass::Expression(ref operand1, ref operator, ref operand2) => {
                list.push_back(format!(
//                     "({} {} {})",
                    "{} {} {}",
                    get_top!(operand1),
                    operator,
                    get_top!(operand2)
                ));
            }
            ASTClass::Identifire(ref id) => {
                list.push_back(format!("{}", id));
            }
            ASTClass::ModulePort(ref id, ref port) => {
                list.push_back(format!("{}.{}", id, port));
            }
            ASTClass::FuncCall(ref id, ref args, ref second_some) => {
                let arg_str = args
                    .iter()
                    .map(|id| format!("{}", get_top!(id)))
                    .collect::<Vec<String>>()
                    .join(", ");

                if let Some(second) = second_some {
                    list.push_back(format!("{}.{}({})", id, second, arg_str));
                } else {
                    list.push_back(format!("{}({})", id, arg_str));
                }
            }
            ASTClass::Number(ref num) => {
                list.push_back(format!("{}", num));
            }
            ASTClass::String(ref _id) => {
                not_implemented!();
            }
            ASTClass::Submodule(ref submodule, ref contents) => {
                let l: Vec<String> = contents
                    .iter()
                    .map(|ref r| {
                        let mut def = format!("{}", r.0);
                        if let Some(ref width) = r.1 {
                            def.push_str(&format!("[{}]", get_top!(width)));
                        }
                        return def;
                    })
                    .collect();
                list.push_back(format!("{} {}", submodule, l.join(", ")));
            }
            ASTClass::BitSlice(ref msb, ref some_lsb) => {
                let m = get_top!(msb);
                if let Some(lsb) = some_lsb {
                    list.push_back(format!("{}:{}", m, get_top!(lsb)));
                } else {
                    list.push_back(format!("{}", m));
                }
            }
            ASTClass::BitslicedExpr(ref expr, ref bitslice) => {
                list.push_back(format!("{}[{}]", get_top!(expr), get_top!(bitslice)));
            }
            ASTClass::FuncIn(ref id, ref args, ref result) => {
                let arg_str = args
                    .iter()
                    .map(|id| format!("{}", id))
                    .collect::<Vec<String>>()
                    .join(", ");
                if let Some(return_port) = result
                {
                    list.push_back(
                        format!("func_in {}({}) : {}", id, arg_str, return_port));
                }
                else
                {
                    list.push_back(format!("func_in {}({})", id, arg_str));
                }
            }
            ASTClass::FuncOut(ref _id, ref _args, ref _result) => {
                not_implemented!();
            }
            ASTClass::FuncSelf(ref id, ref args, ref result) => {
                let arg_str = args
                    .iter()
                    .map(|id| format!("{}", id))
                    .collect::<Vec<String>>()
                    .join(", ");
                if let Some(return_port) = result
                {
                    list.push_back(
                        format!("func_self {}({}) : {}", id, arg_str, return_port));
                }
                else
                {
                    list.push_back(format!("func_self {}({})", id, arg_str));
                }
            }
            ASTClass::Input(ref id, ref some_expr) => {
                if let Some(expr) = some_expr {
                    list.push_back(format!("input {}[{}]", id, get_top!(expr)));
                }
                else {
                    list.push_back(format!("input {}", id));
                }
            }
            ASTClass::Output(ref id, ref some_expr) => {
                if let Some(expr) = some_expr {
                    list.push_back(format!("output {}[{}]", id, get_top!(expr)));
                }
                else {
                    list.push_back(format!("output {}", id));
                }
            }
            ASTClass::InOut(ref id, ref some_expr) => {
                if let Some(expr) = some_expr {
                    list.push_back(format!("inout {}[{}]", id, get_top!(expr)));
                }
                else {
                    list.push_back(format!("inout {}", id));
                }
            }
            ASTClass::Mem(ref _contents) => {
                not_implemented!();
            }
            ASTClass::Wire(ref contents) => {
                let l: Vec<String> = contents
                    .iter()
                    .map(|ref r| {
                        let mut def = format!("{}", r.0);
                        if let Some(ref width) = r.1 {
                            def.push_str(&format!("[{}]", get_top!(width)));
                        }
                        return def;
                    })
                    .collect();
                list.push_back(format!("wire {}", l.join(", ")));
            }
            ASTClass::Reg(ref contents) => {
                let l: Vec<String> = contents
                    .iter()
                    .map(|ref r| {
                        let mut define = format!("{}", r.0);
                        if let Some(ref width) = r.1 {
                            define.push_str(&format!("[{}]", get_top!(width)))
                        }
                        if let Some(ref init) = r.2 {
                            define.push_str(&format!(" = {}", init));
                        }
                        return define;
                    })
                    .collect();
                list.push_back(format!("reg {}", l.join(", ")));
            }
            //             ASTClass::Newline => {
            //                 not_implemented!()
            //             }
            ASTClass::CPPStyleComment(ref comment) => {
                list.push_back(format!("//{}", comment));
            }
            ASTClass::CStyleComment(ref comments) => {
                list.push_back(format!("/*{}*/", comments.join("\n")));
            }
            ASTClass::ProcName(ref _id, ref _args) => {
                not_implemented!();
            }
            ASTClass::StateName(ref ids) => {
                let ids_str = ids
                    .iter()
                    .map(|id_node| format!("{}", id_node))
                    .collect::<Vec<String>>()
                    .join(", ");
                list.push_back(format!("state_name {}", ids_str));
            }
            ASTClass::Assign(ref id, ref expr) => {
                list.push_back(format!("{} = {}", get_top!(id), get_top!(expr)));
            }
            ASTClass::RegAssign(ref id, ref expr) => {
                list.push_back(format!("{} := {}", id, get_top!(expr)));
            }
            ASTClass::Func(ref id, ref func, ref block) => {
                if let Some(fname) = func {
                    list.push_back(format!("func {}.{}", id, fname));
                } else {
                    list.push_back(format!("func {}", id));
                }
                list.append(&mut block.generate());
            }
            ASTClass::Return(ref value) => {
                list.push_back(format!("return {}", get_top!(value)));
            }
            ASTClass::Goto(ref id) => {
                list.push_back(format!("goto {}", id));
            }
            ASTClass::State(ref id, ref block) => {
                list.push_back(format!("state {}", id));
                list.append(&mut block.generate());
            }
            ASTClass::If(ref expr, ref if_block, ref else_block) => {
                list.push_back(format!("if ({})", get_top!(expr)));
                list.append(&mut if_block.generate());
                if let Some(block) = else_block {
                    list.push_back(format!("else"));
                    list.append(&mut block.generate());
                }
            }
            ASTClass::InOut(ref _id, ref _expr) => {
                not_implemented!();
            }
            ASTClass::Operator(ref _op) => {
                not_implemented!();
            }
            ASTClass::UnaryOperator(ref op) => {
                list.push_back(format!("{}", op));
            }
            ASTClass::UnaryOperation(ref a, ref b) => {
                list.push_back(format!("{}{}", get_top!(a), get_top!(b)));
            }
            ASTClass::MacroDefine(ref id, ref value) => {
                if let Some(v) = value {
                    list.push_back(format!("#define {} {}", id, v));
                }
                else
                {
                    list.push_back(format!("#define {}", id));
                }
            }
            ASTClass::MacroInclude(ref path) => {
                list.push_back(format!("#include {}", path));
            }
            ASTClass::MacroIfdef(ref _id) => {
                not_implemented!();
            }
            ASTClass::MacroIfndef(ref _id) => {
                not_implemented!();
            }
            ASTClass::MacroElse => {
                not_implemented!();
            }
            ASTClass::MacroEndif => {
                not_implemented!();
            }
            ASTClass::MacroUndef(ref _id) => {
                not_implemented!();
            }
            ASTClass::EndOfProgram => {
                not_implemented!();
            }
        }
        list
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.class {
            ASTClass::Identifire(ref s) => {
                return write!(f, "{}", s);
            }
            ASTClass::Number(ref num) => {
                return write!(f, "{}", num);
            }
            ASTClass::String(ref path) => {
                return write!(f, "\"{}\"", path);
            }
            ASTClass::Operator(ref op) => write!(f, "{}", op),
            ASTClass::UnaryOperator(ref uop) => write!(f, "{}", uop),
            ASTClass::UnaryOperation(ref a, ref b) => write!(f, "{}{}", a, b),
            _ => {
                panic!(
                    "For the node {:?}, fmt::Display does not implemented yet.",
                    self
                );
            }
        }
    }
}
