use crate::ast::{AbstractSyntaxTree, Document};
use crate::opcode::OpCode;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub functions: Vec<Function>,
    pub constants: Vec<Constant>,
}

#[derive(Debug, Clone)]
pub struct Constant {
    pub name: String,
    pub type_annot: String,
    pub value: String,
}
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub type_annot: String,
}
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub variables: Vec<Variable>,
}

pub fn code_gen_document(
    document: Document,
    symbol_table: &mut SymbolTable,
    op_codes: &mut Vec<OpCode>,
) {
    // There should only be one function(main) in the document for now
    let main_function = document.functions.first().unwrap();

    for statement in &main_function.body {
        code_gen(statement, symbol_table, op_codes);
    }

    // Again, for now, we just halt the program after the main function
    op_codes.push(OpCode::Halt);
}

pub fn code_gen(
    ast: &AbstractSyntaxTree,
    symbol_table: &mut SymbolTable,
    op_codes: &mut Vec<OpCode>,
) {
    match ast {
        AbstractSyntaxTree::Let {
            name,
            type_annot: _,
            value,
        } => match *value.to_owned() {
            AbstractSyntaxTree::Name {
                name: var_or_const_name,
            } => {
                match symbol_table
                    .constants
                    .iter()
                    .find(|v| v.name == var_or_const_name)
                {
                    Some(constant) => {
                        let target_index = symbol_table
                            .functions
                            .last()
                            .unwrap()
                            .variables
                            .iter()
                            .position(|v| v.name == name.to_owned())
                            .unwrap();
                        op_codes.push(OpCode::LoadIntConst {
                            arg1: 1,
                            arg2: constant.value.parse().unwrap(),
                        });
                        op_codes.push(OpCode::Store {
                            arg1: target_index,
                            arg2: 1,
                        });
                    }
                    None => {
                        let var_index = symbol_table
                            .functions
                            .last()
                            .unwrap()
                            .variables
                            .iter()
                            .position(|v| v.name == var_or_const_name)
                            .unwrap();
                        let target_index = symbol_table
                            .functions
                            .last()
                            .unwrap()
                            .variables
                            .iter()
                            .position(|v| v.name == name.to_owned())
                            .unwrap();
                        op_codes.push(OpCode::Load {
                            arg1: 1,
                            arg2: var_index,
                        });
                        op_codes.push(OpCode::Store {
                            arg1: target_index,
                            arg2: 1,
                        });
                    }
                }
            }
            AbstractSyntaxTree::UpName { name: upname } => match upname.as_str() {
                "True" => {
                    let target_index = symbol_table
                        .functions
                        .last()
                        .unwrap()
                        .variables
                        .iter()
                        .position(|v| v.name == name.to_owned())
                        .unwrap();
                    op_codes.push(OpCode::StoreIntConst {
                        arg1: target_index,
                        arg2: 1,
                    });
                }
                "False" => {
                    let target_index = symbol_table
                        .functions
                        .last()
                        .unwrap()
                        .variables
                        .iter()
                        .position(|v| v.name == name.to_owned())
                        .unwrap();
                    op_codes.push(OpCode::StoreIntConst {
                        arg1: target_index,
                        arg2: 0,
                    });
                }
                _ => panic!("Invalid value"),
            },
            AbstractSyntaxTree::Int { value } => {
                let target_index = symbol_table
                    .functions
                    .last()
                    .unwrap()
                    .variables
                    .iter()
                    .position(|v| v.name == name.to_owned())
                    .unwrap();
                op_codes.push(OpCode::StoreIntConst {
                    arg1: target_index,
                    arg2: value,
                });
            }
            AbstractSyntaxTree::Float { value } => {
                let target_index = symbol_table
                    .functions
                    .last()
                    .unwrap()
                    .variables
                    .iter()
                    .position(|v| v.name == name.to_owned())
                    .unwrap();
                op_codes.push(OpCode::StoreFloatConst {
                    arg1: target_index,
                    arg2: value,
                });
            }
            AbstractSyntaxTree::String { value } => {
                let target_index = symbol_table
                    .functions
                    .last()
                    .unwrap()
                    .variables
                    .iter()
                    .position(|v| v.name == name.to_owned())
                    .unwrap();
                op_codes.push(OpCode::StoreStringConst {
                    arg1: target_index,
                    arg2: value.into(),
                });
            }
            AbstractSyntaxTree::Call {
                name: calling_fn_name,
                args,
            } => {
                // if builtin function then generate code for it, if not then panic
                match calling_fn_name.as_str() {
                    "add" => {
                        let arg1 = match args.first().unwrap() {
                            AbstractSyntaxTree::Int { value } => *value,
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => {
                                        let constant_value = constant.value.parse().unwrap();
                                        op_codes.push(OpCode::LoadIntConst {
                                            arg1: 1,
                                            arg2: constant_value,
                                        });
                                        1
                                    }
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 1,
                                            arg2: var_index,
                                        });
                                        1
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let arg2 = match args.last().unwrap() {
                            AbstractSyntaxTree::Int { value } => *value,
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => {
                                        let constant_value = constant.value.parse().unwrap();
                                        op_codes.push(OpCode::LoadIntConst {
                                            arg1: 2,
                                            arg2: constant_value,
                                        });
                                        2
                                    }
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 2,
                                            arg2: var_index,
                                        });
                                        2
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let target_index = symbol_table
                            .functions
                            .last()
                            .unwrap()
                            .variables
                            .iter()
                            .position(|v| v.name == name.to_owned())
                            .unwrap();
                        op_codes.push(OpCode::Add {
                            arg1: 3,
                            arg2: arg1,
                            arg3: arg2,
                        });
                        op_codes.push(OpCode::Store {
                            arg1: target_index,
                            arg2: 3,
                        });
                    }
                    "sub" => {
                        let arg1 = match args.first().unwrap() {
                            AbstractSyntaxTree::Int { value } => *value,
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => constant.value.parse().unwrap(),
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 1,
                                            arg2: var_index,
                                        });
                                        1
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let arg2 = match args.last().unwrap() {
                            AbstractSyntaxTree::Int { value } => *value,
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => constant.value.parse().unwrap(),
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 2,
                                            arg2: var_index,
                                        });
                                        2
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let target_index = symbol_table
                            .functions
                            .last()
                            .unwrap()
                            .variables
                            .iter()
                            .position(|v| v.name == name.to_owned())
                            .unwrap();
                        op_codes.push(OpCode::Sub {
                            arg1: 3,
                            arg2: arg1,
                            arg3: arg2,
                        });
                        op_codes.push(OpCode::Store {
                            arg1: target_index,
                            arg2: 3,
                        });
                    }
                    "add_float" => {
                        let arg1 = match args.first().unwrap() {
                            AbstractSyntaxTree::Float { value } => {
                                op_codes.push(OpCode::LoadFloatConst {
                                    arg1: 1,
                                    arg2: *value,
                                });
                                1
                            }
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => {
                                        op_codes.push(OpCode::LoadFloatConst {
                                            arg1: 1,
                                            arg2: constant.value.parse().unwrap(),
                                        });
                                        1
                                    }
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 1,
                                            arg2: var_index,
                                        });
                                        1
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let arg2 = match args.last().unwrap() {
                            AbstractSyntaxTree::Float { value } => {
                                op_codes.push(OpCode::LoadFloatConst {
                                    arg1: 2,
                                    arg2: *value,
                                });
                                2
                            }
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => {
                                        op_codes.push(OpCode::LoadFloatConst {
                                            arg1: 2,
                                            arg2: constant.value.parse().unwrap(),
                                        });
                                        2
                                    }
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 2,
                                            arg2: var_index,
                                        });
                                        2
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let target_index = symbol_table
                            .functions
                            .last()
                            .unwrap()
                            .variables
                            .iter()
                            .position(|v| v.name == name.to_owned())
                            .unwrap();
                        op_codes.push(OpCode::Add {
                            arg1: 3,
                            arg2: arg1,
                            arg3: arg2,
                        });
                        op_codes.push(OpCode::Store {
                            arg1: target_index,
                            arg2: 3,
                        });
                    }
                    "sub_float" => {
                        let arg1 = match args.first().unwrap() {
                            AbstractSyntaxTree::Float { value } => {
                                op_codes.push(OpCode::LoadFloatConst {
                                    arg1: 1,
                                    arg2: *value,
                                });
                                1
                            }
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => {
                                        op_codes.push(OpCode::LoadFloatConst {
                                            arg1: 1,
                                            arg2: constant.value.parse().unwrap(),
                                        });
                                        1
                                    }
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 1,
                                            arg2: var_index,
                                        });
                                        1
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let arg2 = match args.last().unwrap() {
                            AbstractSyntaxTree::Float { value } => {
                                op_codes.push(OpCode::LoadFloatConst {
                                    arg1: 2,
                                    arg2: *value,
                                });
                                2
                            }
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => {
                                        op_codes.push(OpCode::LoadFloatConst {
                                            arg1: 2,
                                            arg2: constant.value.parse().unwrap(),
                                        });
                                        2
                                    }
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 2,
                                            arg2: var_index,
                                        });
                                        2
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let target_index = symbol_table
                            .functions
                            .last()
                            .unwrap()
                            .variables
                            .iter()
                            .position(|v| v.name == name.to_owned())
                            .unwrap();
                        op_codes.push(OpCode::Sub {
                            arg1: 3,
                            arg2: arg1,
                            arg3: arg2,
                        });
                        op_codes.push(OpCode::Store {
                            arg1: target_index,
                            arg2: 3,
                        });
                    }
                    "concat" => {
                        let arg1 = match args.first().unwrap() {
                            AbstractSyntaxTree::String { value } => {
                                op_codes.push(OpCode::LoadStringConst {
                                    arg1: 1,
                                    arg2: value.clone().into_boxed_str(),
                                });
                                1
                            }
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => {
                                        op_codes.push(OpCode::LoadStringConst {
                                            arg1: 1,
                                            arg2: constant.value.clone().into_boxed_str(),
                                        });
                                        1
                                    }
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 1,
                                            arg2: var_index,
                                        });
                                        1
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let arg2 = match args.last().unwrap() {
                            AbstractSyntaxTree::String { value } => {
                                op_codes.push(OpCode::LoadStringConst {
                                    arg1: 2,
                                    arg2: value.clone().into_boxed_str(),
                                });
                                2
                            }
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => {
                                        op_codes.push(OpCode::LoadStringConst {
                                            arg1: 2,
                                            arg2: constant.value.clone().into_boxed_str(),
                                        });
                                        2
                                    }
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 2,
                                            arg2: var_index,
                                        });
                                        2
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let target_index = symbol_table
                            .functions
                            .last()
                            .unwrap()
                            .variables
                            .iter()
                            .position(|v| v.name == name.to_owned())
                            .unwrap();
                        op_codes.push(OpCode::Concat {
                            arg1: 3,
                            arg2: arg1,
                            arg3: arg2,
                        });
                        op_codes.push(OpCode::Store {
                            arg1: target_index,
                            arg2: 3,
                        });
                    }
                    "and" => {
                        let arg1 = match args.first().unwrap() {
                            AbstractSyntaxTree::UpName { name } => match name.as_str() {
                                "True" => 1,
                                "False" => 0,
                                _ => panic!("Invalid value"),
                            },
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => constant.value.parse().unwrap(),
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 1,
                                            arg2: var_index,
                                        });
                                        1
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let arg2 = match args.last().unwrap() {
                            AbstractSyntaxTree::UpName { name } => match name.as_str() {
                                "True" => 1,
                                "False" => 0,
                                _ => panic!("Invalid value"),
                            },
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => constant.value.parse().unwrap(),
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 2,
                                            arg2: var_index,
                                        });
                                        2
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let target_index = symbol_table
                            .functions
                            .last()
                            .unwrap()
                            .variables
                            .iter()
                            .position(|v| v.name == name.to_owned())
                            .unwrap();
                        op_codes.push(OpCode::And {
                            arg1: 3,
                            arg2: arg1,
                            arg3: arg2,
                        });
                        op_codes.push(OpCode::Store {
                            arg1: target_index,
                            arg2: 3,
                        });
                    }
                    "or" => {
                        let arg1 = match args.first().unwrap() {
                            AbstractSyntaxTree::UpName { name } => match name.as_str() {
                                "True" => 1,
                                "False" => 0,
                                _ => panic!("Invalid value"),
                            },
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => constant.value.parse().unwrap(),
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 1,
                                            arg2: var_index,
                                        });
                                        1
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let arg2 = match args.last().unwrap() {
                            AbstractSyntaxTree::UpName { name } => match name.as_str() {
                                "True" => 1,
                                "False" => 0,
                                _ => panic!("Invalid value"),
                            },
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => constant.value.parse().unwrap(),
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 2,
                                            arg2: var_index,
                                        });
                                        2
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let target_index = symbol_table
                            .functions
                            .last()
                            .unwrap()
                            .variables
                            .iter()
                            .position(|v| v.name == name.to_owned())
                            .unwrap();
                        op_codes.push(OpCode::Or {
                            arg1: 3,
                            arg2: arg1,
                            arg3: arg2,
                        });
                        op_codes.push(OpCode::Store {
                            arg1: target_index,
                            arg2: 3,
                        });
                    }
                    "not" => {
                        let value = match args.first().unwrap() {
                            AbstractSyntaxTree::UpName { name } => match name.as_str() {
                                "True" => 1,
                                "False" => 0,
                                _ => panic!("Invalid value"),
                            },
                            AbstractSyntaxTree::Name { name } => {
                                match symbol_table.constants.iter().find(|v| v.name == *name) {
                                    Some(constant) => constant.value.parse().unwrap(),
                                    None => {
                                        let var_index = symbol_table
                                            .functions
                                            .last()
                                            .unwrap()
                                            .variables
                                            .iter()
                                            .position(|v| v.name == *name)
                                            .unwrap();
                                        op_codes.push(OpCode::Load {
                                            arg1: 1,
                                            arg2: var_index,
                                        });
                                        1
                                    }
                                }
                            }
                            _ => panic!("Invalid value"),
                        };
                        let target_index = symbol_table
                            .functions
                            .last()
                            .unwrap()
                            .variables
                            .iter()
                            .position(|v| v.name == name.to_owned())
                            .unwrap();
                        op_codes.push(OpCode::Not { value });
                        op_codes.push(OpCode::Store {
                            arg1: target_index,
                            arg2: 1,
                        });
                    }
                    _ => panic!("Invalid function name"),
                }
            }
            value => panic!("Invalid value: {:?}", value),
        },
        AbstractSyntaxTree::Block { statements } => {
            for statement in statements {
                code_gen(statement, symbol_table, op_codes);
            }
        }
        AbstractSyntaxTree::Call { name, args } => match name.as_str() {
            "print_integer" => match args.first() {
                Some(arg) => match arg {
                    AbstractSyntaxTree::Int { value } => {
                        op_codes.push(OpCode::LoadIntConst {
                            arg1: 1,
                            arg2: *value,
                        });
                        op_codes.push(OpCode::Print { arg1: 1 });
                    }
                    AbstractSyntaxTree::Name { name } => {
                        match symbol_table.constants.iter().find(|v| v.name == *name) {
                            Some(constant) => {
                                op_codes.push(OpCode::LoadIntConst {
                                    arg1: 1,
                                    arg2: constant.value.parse().unwrap(),
                                });
                                op_codes.push(OpCode::Print { arg1: 1 });
                            }
                            None => {
                                let var_index = symbol_table
                                    .functions
                                    .last()
                                    .unwrap()
                                    .variables
                                    .iter()
                                    .position(|v| v.name == *name)
                                    .unwrap();
                                op_codes.push(OpCode::Load {
                                    arg1: 1,
                                    arg2: var_index,
                                });
                                op_codes.push(OpCode::Print { arg1: 1 });
                            }
                        }
                    }
                    _ => panic!("Invalid value"),
                },
                None => panic!("No args"),
            },
            "print_float" => match args.first() {
                Some(arg) => match arg {
                    AbstractSyntaxTree::Float { value } => {
                        op_codes.push(OpCode::LoadFloatConst {
                            arg1: 1,
                            arg2: *value,
                        });
                        op_codes.push(OpCode::Print { arg1: 1 });
                    }
                    AbstractSyntaxTree::Name { name } => {
                        match symbol_table.constants.iter().find(|v| v.name == *name) {
                            Some(constant) => {
                                op_codes.push(OpCode::LoadFloatConst {
                                    arg1: 1,
                                    arg2: constant.value.parse().unwrap(),
                                });
                                op_codes.push(OpCode::Print { arg1: 1 });
                            }
                            None => {
                                let var_index = symbol_table
                                    .functions
                                    .last()
                                    .unwrap()
                                    .variables
                                    .iter()
                                    .position(|v| v.name == *name)
                                    .unwrap();
                                op_codes.push(OpCode::Load {
                                    arg1: 1,
                                    arg2: var_index,
                                });
                                op_codes.push(OpCode::Print { arg1: 1 });
                            }
                        }
                    }
                    _ => panic!("Invalid value"),
                },
                None => panic!("No args"),
            },
            "print_bool" => match args.first() {
                Some(arg) => match arg {
                    AbstractSyntaxTree::UpName { name } => match name.as_str() {
                        "True" => {
                            op_codes.push(OpCode::Print { arg1: 1 });
                        }
                        "False" => {
                            op_codes.push(OpCode::Print { arg1: 0 });
                        }
                        _ => panic!("Invalid value"),
                    },
                    AbstractSyntaxTree::Name { name } => {
                        match symbol_table.constants.iter().find(|v| v.name == *name) {
                            Some(constant) => {
                                op_codes.push(OpCode::LoadIntConst {
                                    arg1: 1,
                                    arg2: constant.value.parse().unwrap(),
                                });
                                op_codes.push(OpCode::Print { arg1: 1 });
                            }
                            None => {
                                let var_index = symbol_table
                                    .functions
                                    .last()
                                    .unwrap()
                                    .variables
                                    .iter()
                                    .position(|v| v.name == *name)
                                    .unwrap();
                                op_codes.push(OpCode::Load {
                                    arg1: 1,
                                    arg2: var_index,
                                });
                                op_codes.push(OpCode::Print { arg1: 1 });
                            }
                        }
                    }
                    _ => panic!("Invalid value"),
                },
                None => panic!("No args"),
            },
            "print_string" => match args.first() {
                Some(arg) => match arg {
                    AbstractSyntaxTree::String { value } => {
                        op_codes.push(OpCode::LoadStringConst {
                            arg1: 1,
                            arg2: value.clone().into_boxed_str(),
                        });
                        op_codes.push(OpCode::Print { arg1: 1 });
                    }
                    AbstractSyntaxTree::Name { name } => {
                        let var_index = symbol_table
                            .functions
                            .last()
                            .unwrap()
                            .variables
                            .iter()
                            .position(|v| v.name == *name)
                            .unwrap();
                        op_codes.push(OpCode::Load {
                            arg1: 1,
                            arg2: var_index,
                        });
                        op_codes.push(OpCode::Print { arg1: 1 });
                    }
                    _ => panic!("Invalid value"),
                },
                None => panic!("No args"),
            },
            _ => panic!("Invalid function name"),
        },
        _ => {
            panic!("Invalid code");
        }
    }
}
