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
    WidthBlock(Box<ASTNode>),
    Operator(token::Operator),

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
    Expression(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>),
    Comment(String),
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
                return write!(f, "\ndeclare {}{}", id, interfaces);
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
            ASTClass::Input(ref id, ref expr) => {
                if let ASTClass::Number(ref width) = expr.class {
                    if width == "1" {
                        return write!(f, "input {};\n", id);
                    } else {
                        return write!(f, "input {}[{}];\n", id, expr);
                    }
                }
                return write!(f, "input {}[{}];\n", id, expr);
            }
            ASTClass::Output(ref id, ref expr) => {
                if let ASTClass::Number(ref width) = expr.class {
                    if width == "1" {
                        return write!(f, "output {};\n", id);
                    } else {
                        return write!(f, "output {}[{}];\n", id, expr);
                    }
                }
                return write!(f, "output {}[{}];\n", id, expr);
            }
            ASTClass::FuncIn(ref id, ref input_ids, ref output) => {
                let str_input: Vec<String> =
                    input_ids.iter().map(|ident| format!("{}", ident)).collect();
                let args = str_input.connect(", ");

                if let ASTClass::Identifire(ref s) = output.class {
                    if s.is_empty() {
                        return write!(f, "func_in {}({});\n", id, args);
                    } else {
                        return write!(f, "func_in {}({}) : {};\n", id, args, output);
                    }
                } else {
                    panic!("UnExpectedToken at {}", line!());
                }
            }
            ASTClass::FuncOut(ref id, ref input_ids, ref output) => {
                let str_input: Vec<String> =
                    input_ids.iter().map(|ident| format!("{}", ident)).collect();
                let args = str_input.connect(", ");

                if let ASTClass::Identifire(ref s) = output.class {
                    if s.is_empty() {
                        return write!(f, "func_out {}({});\n", id, args);
                    } else {
                        return write!(f, "func_out {}({}) : {};\n", id, args, output);
                    }
                } else {
                    panic!("UnExpectedToken at {}", line!());
                }
            }
            ASTClass::Block(ref list, nest) => {
                let mut list_str = String::new();
                let nest_tabs = "\t".repeat(nest);
                let mut double_newline_flag = false;
                for node in list {
                    match node.class {
                        ASTClass::Newline => {
                            if double_newline_flag {
                            list_str.push_str("\n");
                            double_newline_flag = false;
                            }
                            else {
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

                return write!(f, "\n{{\n{}}}\n\n", list_str);
            }
            ASTClass::MacroInclude(ref path) => {
                return write!(f, "#include {}\n", path);
            }
            ASTClass::MacroIfndef(ref id) => {
                return write!(f, "#ifndef {}\n", id);
            }
            ASTClass::MacroDefine(ref id, ref string) => {
                if string.len() == 0 {
                    return write!(f, "#define {}\n", id);
                }
                else {
                    return write!(f, "#define {} {}\n", id, string);
                }
            }
            ASTClass::MacroEndif => {
                return write!(f, "#endif\n");
            }
            ASTClass::EndOfProgram => {
                return write!(f, "");
            }
            ASTClass::Newline => {
                return write!(f, "\n");
            }
            ASTClass::Comment(ref line) => {
                return write!(f, "// {}\n", line);
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
