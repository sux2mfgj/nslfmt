use std::fmt;
use token;

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
    Block(Vec<Box<ASTNode>>, usize),
    Operator(token::Operator),

    // ----- Declare ------
    // identifire, block
    Declare(Box<ASTNode>, Box<ASTNode>),

    // identifire, inputs, output
    FuncIn(Box<ASTNode>, Option<Vec<Box<ASTNode>>>, Option<Box<ASTNode>>),
    // identifire, outputs, input
    FuncOut(Box<ASTNode>, Option<Vec<Box<ASTNode>>>, Option<Box<ASTNode>>),
    // identifire, inputs, output
    FuncSelf(
        Box<ASTNode>,
        Option<Vec<Box<ASTNode>>>,
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
    Macro_SubModule(Vec<token::Token>),
    ProcName(Box<ASTNode>, Option<Vec<Box<ASTNode>>>),
    StateName(Vec<String>),
    //  id          ,[12]        , [12]                 , initial value
    Mem(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>, Option<Vec<Box<ASTNode>>>),

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

    //          operand     , operation   , operand
    Expression(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>),
    CStyleComment(String),
    CPPStyleComment(Vec<String>),
    Newline,
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
                return write!(f, "declare {}{}", id, interfaces);
            }
            ASTClass::Module(ref id, ref contents) => {
                return write!(f, "module {}{}", id, contents);
            }
            ASTClass::Identifire(ref s) => {
                return write!(f, "{}", s);
            }
            ASTClass::Number(ref num) => {
                return write!(f, "{}", num);
            }
            ASTClass::String(ref path) => {
                return write!(f, "\"{}\"", path);
            }
            ASTClass::Operator(ref op) => {
                return write!(f, "{}", op);
            }
            ASTClass::Expression(ref operand1, ref operator, ref operand2) => {
                return write!(f, "{} {} {}", operand1, operator, operand2)
            }
            ASTClass::Wire(ref list) => {
                let id_list: Vec<String> = list
                    .iter()
                    .map(|def| match def.1 {
                        Some(ref w) => {
                            return format!("{}[{}]", def.0, w);
                        }
                        None => {
                            return format!("{}", def.0);
                        }
                    })
                    .collect();
                let def_str = id_list.join(", ");
                return write!(f, "wire {};\n", def_str);
            }
            ASTClass::Input(ref id, ref expr) => match expr {
                Some(width) => {
                    return write!(f, "input {}[{}];\n", id, width);
                }
                None => {
                    return write!(f, "input {};\n", id);
                }
            },
            ASTClass::Output(ref id, ref expr) => match expr {
                Some(width) => {
                    return write!(f, "output {}[{}];\n", id, width);
                }
                None => {
                    return write!(f, "output {};\n", id);
                }
            },
            ASTClass::FuncIn(ref id, ref input_ids, ref output) => {
                let mut args = "".to_string();
                match input_ids {
                    Some(ids) => {
                        let str_input: Vec<String> =
                            ids.iter().map(|ident| format!("{}", ident)).collect();
                        //let args = str_input.connect(", ");
                        args = str_input.join(", ");

                    }
                    None => {
                    }
                }
                match output {
                    Some(s) => {
                        return write!(f, "func_in {}({}) : {};\n", id, args, s);
                    }
                    None => {
                        return write!(f, "func_in {}({});\n", id, args);
                    }
                }
            }
            ASTClass::FuncOut(ref id, ref input_ids, ref output) => {
                let mut args = "".to_string();
                match input_ids {
                    Some(ids) => {
                        let str_input: Vec<String> =
                            ids.iter().map(|ident| format!("{}", ident)).collect();
                        args = str_input.join(", ");
                    }
                    None => {}
                }
                match output {
                    Some(s) => {
                        return write!(f, "func_out {}({}) : {};\n", id, args, s);
                    }
                    None => {
                        return write!(f, "func_out {}({});\n", id, args);
                    }
                }
            }
            ASTClass::Block(ref list, nest) => {
                let mut list_str = String::new();
                let nest_tabs = "    ".repeat(nest);
                let mut double_newline_flag = false;
                for node in list {
                    match node.class {
                        ASTClass::Newline => {
                            if double_newline_flag {
                                list_str.push_str("\n");
                                double_newline_flag = false;
                            } else {
                                double_newline_flag = true;
                                continue;
                            }
                        }
                        ASTClass::Identifire(ref id) => {
                            double_newline_flag = false;
                            list_str.push_str(&format!("{}{}\n", nest_tabs, id));
                        }
                        _ => {
                            double_newline_flag = false;
                            list_str.push_str(&format!("{}{}", nest_tabs, node));
                        }
                    }
                }

                return write!(f, "\n{{\n{}}}\n", list_str);
            }
            ASTClass::MacroInclude(ref path) => {
                return write!(f, "#include {}\n", path);
            }
            ASTClass::MacroIfndef(ref id) => {
                return write!(f, "#ifndef {}\n", id);
            }
            ASTClass::MacroDefine(ref id, ref string) => match string {
                Some(s) => {
                    return write!(f, "#define {} {}\n", id, s);
                }
                None => {
                    return write!(f, "#define {}\n", id);
                }
            },
            ASTClass::MacroEndif => {
                return write!(f, "#endif\n");
            }
            ASTClass::EndOfProgram => {
                return write!(f, "");
            }
            ASTClass::Newline => {
                return write!(f, "");
            }
            ASTClass::CStyleComment(ref line) => {
                return write!(f, "// {}\n", line);
            }
            ASTClass::CPPStyleComment(ref list) => {
                return write!(f, "/*{}*/\n", list.join("\n"));
            }
            _ => {
                panic!(
                    "For the node {:?}, fmt::Display does not implemented yet.",
                    self
                );
            }
        }
    }
}
