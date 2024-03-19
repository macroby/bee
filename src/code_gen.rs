use crate::ast::AbstractSyntaxTree;
use crate::opcode::OpCode;

#[derive(Debug)]
pub struct SymbolTable {
    pub variables: Vec<Variable>,
    pub functions: Vec<Function>,
    pub constants: Vec<Constant>,
}

#[derive(Debug)]
pub struct Constant {
    pub name: String,
    pub type_annot: String,
    pub value: String,
}
#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub type_annot: String,
}
#[derive(Debug)]
pub struct Function {
    pub name: String,
}

pub fn code_gen(
    ast: AbstractSyntaxTree,
    symbol_table: &mut SymbolTable,
    op_codes: &mut Vec<OpCode>,
) {
    match ast {
        AbstractSyntaxTree::Const { .. } => {}
        AbstractSyntaxTree::Let {
            name,
            type_annot: _,
            value,
        } => match *value {
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
                            .variables
                            .iter()
                            .position(|v| v.name == name)
                            .unwrap();
                        op_codes.push(OpCode::LoadImm {
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
                            .variables
                            .iter()
                            .position(|v| v.name == var_or_const_name)
                            .unwrap();
                        let target_index = symbol_table
                            .variables
                            .iter()
                            .position(|v| v.name == name)
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
                        .variables
                        .iter()
                        .position(|v| v.name == name)
                        .unwrap();
                    op_codes.push(OpCode::StoreImm {
                        arg1: target_index,
                        arg2: 1,
                    });
                }
                "False" => {
                    let target_index = symbol_table
                        .variables
                        .iter()
                        .position(|v| v.name == name)
                        .unwrap();
                    op_codes.push(OpCode::StoreImm {
                        arg1: target_index,
                        arg2: 0,
                    });
                }
                _ => panic!("Invalid value"),
            },
            AbstractSyntaxTree::Int { value } => {
                let target_index = symbol_table
                    .variables
                    .iter()
                    .position(|v| v.name == name)
                    .unwrap();
                op_codes.push(OpCode::StoreImm {
                    arg1: target_index,
                    arg2: value,
                });
            }
            AbstractSyntaxTree::Minus { left, right } => {
                match *left {
                    AbstractSyntaxTree::Int { value } => {
                        op_codes.push(OpCode::LoadImm {
                            arg1: 1,
                            arg2: value as usize,
                        });
                    }
                    AbstractSyntaxTree::Name { name } => {
                        let var_index = symbol_table
                            .variables
                            .iter()
                            .position(|v| v.name == name)
                            .unwrap();
                        op_codes.push(OpCode::Load {
                            arg1: 1,
                            arg2: var_index,
                        });
                    }
                    _ => panic!("Invalid value"),
                }

                match *right {
                    AbstractSyntaxTree::Int { value } => {
                        op_codes.push(OpCode::LoadImm {
                            arg1: 2,
                            arg2: value as usize,
                        });
                    }
                    AbstractSyntaxTree::Name { name } => {
                        let var_index = symbol_table
                            .variables
                            .iter()
                            .position(|v| v.name == name)
                            .unwrap();
                        op_codes.push(OpCode::Load {
                            arg1: 2,
                            arg2: var_index,
                        });
                    }
                    _ => panic!("Invalid value"),
                }

                op_codes.push(OpCode::Sub {
                    arg1: 3,
                    arg2: 1,
                    arg3: 2,
                });

                let target_index = symbol_table
                    .variables
                    .iter()
                    .position(|v| v.name == name)
                    .unwrap();

                op_codes.push(OpCode::Store {
                    arg1: target_index,
                    arg2: 3,
                });
            }
            AbstractSyntaxTree::Plus { left, right } => {
                match *left {
                    AbstractSyntaxTree::Int { value } => {
                        op_codes.push(OpCode::LoadImm {
                            arg1: 1,
                            arg2: value as usize,
                        });
                    }
                    AbstractSyntaxTree::Name { name } => {
                        let var_index = symbol_table
                            .variables
                            .iter()
                            .position(|v| v.name == name)
                            .unwrap();
                        op_codes.push(OpCode::Load {
                            arg1: 1,
                            arg2: var_index,
                        });
                    }
                    _ => panic!("Invalid value"),
                }

                match *right {
                    AbstractSyntaxTree::Int { value } => {
                        op_codes.push(OpCode::LoadImm {
                            arg1: 2,
                            arg2: value as usize,
                        });
                    }
                    AbstractSyntaxTree::Name { name } => {
                        let var_index = symbol_table
                            .variables
                            .iter()
                            .position(|v| v.name == name)
                            .unwrap();
                        op_codes.push(OpCode::Load {
                            arg1: 2,
                            arg2: var_index,
                        });
                    }
                    _ => panic!("Invalid value"),
                }

                op_codes.push(OpCode::Add {
                    arg1: 3,
                    arg2: 1,
                    arg3: 2,
                });

                let target_index = symbol_table
                    .variables
                    .iter()
                    .position(|v| v.name == name)
                    .unwrap();

                op_codes.push(OpCode::Store {
                    arg1: target_index,
                    arg2: 3,
                });
            }
            AbstractSyntaxTree::And { left, right } => {
                match *left {
                    AbstractSyntaxTree::UpName { name } => {
                        if name == "True" {
                            op_codes.push(OpCode::LoadImm { arg1: 1, arg2: 1 });
                        } else if name == "False" {
                            op_codes.push(OpCode::LoadImm { arg1: 1, arg2: 0 });
                        } else {
                            panic!("Invalid value");
                        }
                    }
                    AbstractSyntaxTree::Name { name } => {
                        let var_index = symbol_table
                            .variables
                            .iter()
                            .position(|v| v.name == name)
                            .unwrap();
                        op_codes.push(OpCode::Load {
                            arg1: 1,
                            arg2: var_index,
                        });
                    }
                    _ => panic!("Invalid value"),
                }

                match *right {
                    AbstractSyntaxTree::UpName { name } => {
                        if name == "True" {
                            op_codes.push(OpCode::LoadImm { arg1: 2, arg2: 1 });
                        } else if name == "False" {
                            op_codes.push(OpCode::LoadImm { arg1: 2, arg2: 0 });
                        } else {
                            panic!("Invalid value");
                        }
                    }
                    AbstractSyntaxTree::Name { name } => {
                        let var_index = symbol_table
                            .variables
                            .iter()
                            .position(|v| v.name == name)
                            .unwrap();
                        op_codes.push(OpCode::Load {
                            arg1: 2,
                            arg2: var_index,
                        });
                    }
                    _ => panic!("Invalid value"),
                }

                op_codes.push(OpCode::And {
                    arg1: 3,
                    arg2: 1,
                    arg3: 2,
                });

                let target_index = symbol_table
                    .variables
                    .iter()
                    .position(|v| v.name == name)
                    .unwrap();

                op_codes.push(OpCode::Store {
                    arg1: target_index,
                    arg2: 3,
                });
            }
            AbstractSyntaxTree::Or { left, right } => {
                match *left {
                    AbstractSyntaxTree::UpName { name } => {
                        if name == "True" {
                            op_codes.push(OpCode::LoadImm { arg1: 1, arg2: 1 });
                        } else if name == "False" {
                            op_codes.push(OpCode::LoadImm { arg1: 1, arg2: 0 });
                        } else {
                            panic!("Invalid value");
                        }
                    }
                    AbstractSyntaxTree::Name { name: var_name } => {
                        let var_index = symbol_table
                            .variables
                            .iter()
                            .position(|v| v.name == var_name)
                            .unwrap();
                        op_codes.push(OpCode::Load {
                            arg1: 1,
                            arg2: var_index,
                        });
                    }
                    _ => panic!("Invalid value"),
                }

                match *right {
                    AbstractSyntaxTree::UpName { name } => {
                        if name == "True" {
                            op_codes.push(OpCode::LoadImm { arg1: 2, arg2: 1 });
                        } else if name == "False" {
                            op_codes.push(OpCode::LoadImm { arg1: 2, arg2: 0 });
                        } else {
                            panic!("Invalid value");
                        }
                    }
                    AbstractSyntaxTree::Name { name } => {
                        let var_index = symbol_table
                            .variables
                            .iter()
                            .position(|v| v.name == name)
                            .unwrap();
                        op_codes.push(OpCode::Load {
                            arg1: 2,
                            arg2: var_index,
                        });
                    }
                    _ => panic!("Invalid value"),
                }

                op_codes.push(OpCode::Or {
                    arg1: 3,
                    arg2: 1,
                    arg3: 2,
                });

                let target_index = symbol_table
                    .variables
                    .iter()
                    .position(|v| v.name == name)
                    .unwrap();

                op_codes.push(OpCode::Store {
                    arg1: target_index,
                    arg2: 3,
                });
            }
            AbstractSyntaxTree::Not { value } => {
                match *value {
                    AbstractSyntaxTree::UpName { name } => {
                        if name == "True" {
                            op_codes.push(OpCode::LoadImm { arg1: 1, arg2: 1 });
                        } else if name == "False" {
                            op_codes.push(OpCode::LoadImm { arg1: 1, arg2: 0 });
                        } else {
                            panic!("Invalid value");
                        }
                    }
                    AbstractSyntaxTree::Name { name } => {
                        let var_index = symbol_table
                            .variables
                            .iter()
                            .position(|v| v.name == name)
                            .unwrap();
                        op_codes.push(OpCode::Load {
                            arg1: 1,
                            arg2: var_index,
                        });
                    }
                    _ => panic!("Invalid value"),
                }

                op_codes.push(OpCode::Not { value: 1 });

                let target_index = symbol_table
                    .variables
                    .iter()
                    .position(|v| v.name == name)
                    .unwrap();

                op_codes.push(OpCode::Store {
                    arg1: target_index,
                    arg2: 1,
                });
            }
            value => panic!("Invalid value: {:?}", value),
        },
        AbstractSyntaxTree::Fn { name, body } => {
            symbol_table.functions.push(Function { name: name.clone() });
            for statement in *body {
                code_gen(statement, symbol_table, op_codes);
            }
        }
        AbstractSyntaxTree::Block { statements } => {
            for statement in statements {
                code_gen(statement, symbol_table, op_codes);
            }
            op_codes.push(OpCode::Halt);
        }
        AbstractSyntaxTree::Call { name, args } => match name.as_str() {
            "print_integer" => match args.first() {
                Some(arg) => match arg {
                    AbstractSyntaxTree::Int { value } => {
                        op_codes.push(OpCode::Print {
                            arg1: *value as usize,
                        });
                    }
                    AbstractSyntaxTree::Name { name } => {
                        match symbol_table.constants.iter().find(|v| v.name == *name) {
                            Some(constant) => {
                                op_codes.push(OpCode::LoadImm {
                                    arg1: 1,
                                    arg2: constant.value.parse().unwrap(),
                                });
                                op_codes.push(OpCode::Print { arg1: 1 });
                            }
                            None => {
                                let var_index = symbol_table
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
                                op_codes.push(OpCode::LoadImm {
                                    arg1: 1,
                                    arg2: constant.value.parse().unwrap(),
                                });
                                op_codes.push(OpCode::Print { arg1: 1 });
                            }
                            None => {
                                let var_index = symbol_table
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
            _ => panic!("Invalid function name"),
        },
        _ => {
            panic!("Invalid code");
        }
    }
}