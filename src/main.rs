use core::panic;
use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("test.bee").expect("Should have been able to read the file");
    let tokens = lex(contents.clone());
    let ast = parse(tokens);
    println!("AST:");
    println!("{:?}", ast);
    let mut symbol_table = SymbolTable {
        variables: Vec::new(),
        functions: Vec::new(),
        constants: Vec::new(),
    };
    analyze(ast.clone(), &mut symbol_table);
    println!("Symbol Table:");
    println!("{:?}", symbol_table);

    let mut op_codes = Vec::new();
    code_gen(ast.clone(), &mut symbol_table, &mut op_codes);

    println!("Codegen Output:");
    for op_code in op_codes.clone() {
        println!("{:?}", op_code);
    }

    println!("Interpretation:");
    interpret(op_codes);
}

fn interpret(op_codes: Vec<OpCode>) {
    let mut registers = [0; 32];
    let mut variables: HashMap<usize, usize> = HashMap::new();
    let mut pc = 0;
    loop {
        let op_code = &op_codes[pc];
        match op_code {
            OpCode::Add { arg1, arg2, arg3 } => {
                registers[*arg1] = registers[*arg2] + registers[*arg3];
            }
            OpCode::Sub { arg1, arg2, arg3 } => {
                registers[*arg1] = registers[*arg2] - registers[*arg3];
            }
            OpCode::And { arg1, arg2, arg3 } => {
                registers[*arg1] = registers[*arg2] & registers[*arg3];
            }
            OpCode::Or { arg1, arg2, arg3 } => {
                registers[*arg1] = registers[*arg2] | registers[*arg3];
            }
            OpCode::Not { value } => {
                registers[*value] = registers[*value] ^ 1;
            }
            OpCode::Load { arg1, arg2 } => {
                registers[*arg1] = *variables.get(&arg2).unwrap();
            }
            OpCode::LoadImm { arg1, arg2 } => {
                registers[*arg1] = *arg2;
            }
            OpCode::Move { arg1, arg2 } => {
                registers[*arg1] = registers[*arg2];
            }
            OpCode::Store { arg1, arg2 } => {
                variables.insert(*arg1, registers[*arg2]);
            }
            OpCode::StoreImm { arg1, arg2 } => {
                variables.insert(*arg1, *arg2);
            }
            OpCode::Print { arg1 } => {
                println!("{}", registers[*arg1]);
            }
            OpCode::Halt => {
                break;
            }
        }
        pc += 1;
    }
}

fn code_gen(ast: AbstractSyntaxTree, symbol_table: &mut SymbolTable, op_codes: &mut Vec<OpCode>) {
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

fn analyze(ast: AbstractSyntaxTree, symbol_table: &mut SymbolTable) {
    match ast {
        AbstractSyntaxTree::Const {
            name,
            type_annot,
            value,
        } => match *value {
            AbstractSyntaxTree::Int { value: int_value } => {
                if type_annot != "Integer" {
                    panic!("Type mismatch");
                }
                symbol_table.constants.push(Constant {
                    name: name.clone(),
                    type_annot: type_annot.clone(),
                    value: int_value.to_string(),
                });
            }
            AbstractSyntaxTree::String {
                value: string_value,
            } => {
                if type_annot != "String" {
                    panic!("Type mismatch");
                }
                symbol_table.constants.push(Constant {
                    name: name.clone(),
                    type_annot: type_annot.clone(),
                    value: string_value.clone(),
                });
            }
            AbstractSyntaxTree::UpName { name: bool_name } => {
                if type_annot == "Bool" && (bool_name == "True" || bool_name == "False") {
                } else {
                    panic!("Type mismatch");
                }
                symbol_table.constants.push(Constant {
                    name: name.clone(),
                    type_annot: type_annot.clone(),
                    value: bool_name.clone(),
                });
            }
            _ => panic!("Invalid value"),
        },
        AbstractSyntaxTree::Let {
            name,
            type_annot,
            value,
        } => {
            match *value {
                AbstractSyntaxTree::Int { .. } => {
                    if type_annot != "Integer" {
                        panic!("Type mismatch");
                    }
                }
                AbstractSyntaxTree::String { .. } => {
                    if type_annot != "String" {
                        panic!("Type mismatch");
                    }
                }
                AbstractSyntaxTree::UpName { name } => {
                    if type_annot == "Bool" && (name == "True" || name == "False") {
                    } else {
                        panic!("Type mismatch");
                    }
                }
                AbstractSyntaxTree::Name { name } => {
                    let variable = symbol_table.variables.iter().find(|v| v.name == name);
                    match variable {
                        Some(v) => {
                            if v.type_annot != type_annot {
                                panic!("Type mismatch");
                            }
                        }
                        None => panic!("Variable not found"),
                    }
                }
                AbstractSyntaxTree::Plus { left, right } => {
                    match type_annot.as_str() {
                        "Integer" => (),
                        _ => panic!("Invalid type"),
                    }
                    match *left {
                        AbstractSyntaxTree::Int { value: _ } => {
                            if type_annot != "Integer" {
                                panic!("Type mismatch");
                            }
                        }
                        AbstractSyntaxTree::Name { name } => {
                            let variable = symbol_table.variables.iter().find(|v| v.name == name);
                            match variable {
                                Some(v) => {
                                    if v.type_annot != type_annot {
                                        panic!("Type mismatch");
                                    }
                                }
                                None => panic!("Variable not found"),
                            }
                        }
                        _ => panic!("Invalid value"),
                    }
                    match *right {
                        AbstractSyntaxTree::Int { .. } => {
                            if type_annot != "Integer" {
                                panic!("Type mismatch");
                            }
                        }
                        AbstractSyntaxTree::Name { name } => {
                            let variable = symbol_table.variables.iter().find(|v| v.name == name);
                            match variable {
                                Some(v) => {
                                    if v.type_annot != type_annot {
                                        panic!("Type mismatch");
                                    }
                                }
                                None => panic!("Variable not found"),
                            }
                        }
                        _ => panic!("Invalid value"),
                    }
                }
                AbstractSyntaxTree::Minus { left, right } => {
                    match type_annot.as_str() {
                        "Integer" => (),
                        _ => panic!("Invalid type"),
                    }
                    match *left {
                        AbstractSyntaxTree::Int { .. } => {
                            if type_annot != "Integer" {
                                panic!("Type mismatch");
                            }
                        }
                        AbstractSyntaxTree::Name { name } => {
                            let variable = symbol_table.variables.iter().find(|v| v.name == name);
                            match variable {
                                Some(v) => {
                                    if v.type_annot != type_annot {
                                        panic!("Type mismatch");
                                    }
                                }
                                None => panic!("Variable not found"),
                            }
                        }
                        _ => panic!("Invalid value"),
                    }
                    match *right {
                        AbstractSyntaxTree::Int { value: _ } => {
                            if type_annot != "Integer" {
                                panic!("Type mismatch");
                            }
                        }
                        AbstractSyntaxTree::Name { name } => {
                            let variable = symbol_table.variables.iter().find(|v| v.name == name);
                            match variable {
                                Some(v) => {
                                    if v.type_annot != type_annot {
                                        panic!("Type mismatch");
                                    }
                                }
                                None => panic!("Variable not found"),
                            }
                        }
                        _ => panic!("Invalid value"),
                    }
                }
                AbstractSyntaxTree::LtGt { left, right } => {
                    match type_annot.as_str() {
                        "String" => (),
                        _ => panic!("Invalid type"),
                    }
                    match *left {
                        AbstractSyntaxTree::String { .. } => {
                            if type_annot != "String" {
                                panic!("Type mismatch");
                            }
                        }
                        AbstractSyntaxTree::Name { name } => {
                            let variable = symbol_table.variables.iter().find(|v| v.name == name);
                            match variable {
                                Some(v) => {
                                    if v.type_annot != type_annot {
                                        panic!("Type mismatch");
                                    }
                                }
                                None => panic!("Variable not found"),
                            }
                        }
                        _ => panic!("Invalid value"),
                    }
                    match *right {
                        AbstractSyntaxTree::String { value: _ } => {
                            if type_annot != "String" {
                                panic!("Type mismatch");
                            }
                        }
                        AbstractSyntaxTree::Name { name } => {
                            let variable = symbol_table.variables.iter().find(|v| v.name == name);
                            match variable {
                                Some(v) => {
                                    if v.type_annot != type_annot {
                                        panic!("Type mismatch");
                                    }
                                }
                                None => panic!("Variable not found"),
                            }
                        }
                        _ => panic!("Invalid value"),
                    }
                }
                AbstractSyntaxTree::And { left, right }
                | AbstractSyntaxTree::Or { left, right } => {
                    match type_annot.as_str() {
                        "Bool" => (),
                        _ => panic!("Invalid type"),
                    }
                    match *left {
                        AbstractSyntaxTree::UpName { name } => {
                            if name != "True" && name != "False" {
                                panic!("Invalid value");
                            }
                        }
                        AbstractSyntaxTree::Name { name } => {
                            let variable = symbol_table.variables.iter().find(|v| v.name == name);
                            match variable {
                                Some(v) => {
                                    if v.type_annot != type_annot {
                                        panic!("Type mismatch");
                                    }
                                }
                                None => panic!("Variable not found"),
                            }
                        }
                        _ => panic!("Invalid value"),
                    }
                    match *right {
                        AbstractSyntaxTree::UpName { name } => {
                            if name != "True" && name != "False" {
                                panic!("Invalid value");
                            }
                        }
                        AbstractSyntaxTree::Name { name } => {
                            let variable = symbol_table.variables.iter().find(|v| v.name == name);
                            match variable {
                                Some(v) => {
                                    if v.type_annot != type_annot {
                                        panic!("Type mismatch");
                                    }
                                }
                                None => panic!("Variable not found"),
                            }
                        }
                        _ => panic!("Invalid value"),
                    }
                }
                AbstractSyntaxTree::Not { value } => {
                    match type_annot.as_str() {
                        "Bool" => (),
                        _ => panic!("Invalid type"),
                    }
                    match *value {
                        AbstractSyntaxTree::UpName { name } => {
                            if name != "True" && name != "False" {
                                panic!("Invalid value");
                            }
                        }
                        AbstractSyntaxTree::Name { name } => {
                            let variable = symbol_table.variables.iter().find(|v| v.name == name);
                            match variable {
                                Some(v) => {
                                    if v.type_annot != type_annot {
                                        panic!("Type mismatch");
                                    }
                                }
                                None => panic!("Variable not found"),
                            }
                        }
                        _ => panic!("Invalid value"),
                    }
                }
                _ => panic!("Invalid value"),
            }
            symbol_table.variables.push(Variable {
                name: name.clone(),
                type_annot: type_annot.clone(),
            });
        }
        AbstractSyntaxTree::Fn { name, body } => {
            symbol_table.functions.push(Function { name: name.clone() });
            for statement in *body {
                analyze(statement, symbol_table);
            }
        }
        AbstractSyntaxTree::Block { statements } => {
            for statement in statements {
                analyze(statement, symbol_table);
            }
        }
        AbstractSyntaxTree::Plus { left, right } => {
            analyze(*left, symbol_table);
            analyze(*right, symbol_table);
        }
        AbstractSyntaxTree::Minus { left, right } => {
            analyze(*left, symbol_table);
            analyze(*right, symbol_table);
        }
        AbstractSyntaxTree::LtGt { left, right } => {
            analyze(*left, symbol_table);
            analyze(*right, symbol_table);
        }
        AbstractSyntaxTree::And { left, right } => {
            analyze(*left, symbol_table);
            analyze(*right, symbol_table);
        }
        AbstractSyntaxTree::Or { left, right } => {
            analyze(*left, symbol_table);
            analyze(*right, symbol_table);
        }
        AbstractSyntaxTree::Not { value } => {
            analyze(*value, symbol_table);
        }
        _ => {}
    }
}

fn parse(tokens: Vec<Token>) -> AbstractSyntaxTree {
    let mut tokens = tokens.iter().peekable();
    let mut statements = Vec::new();
    while let Some(token) = tokens.next() {
        match token {
            Token::Const => {
                let name = match tokens.next() {
                    Some(Token::Name { name }) => name,
                    _ => panic!("Expected a name after const"),
                };
                let type_annotation: String = match tokens.next() {
                    Some(Token::Colon) => match tokens.next() {
                        Some(Token::UpName { name }) => name.clone(),
                        _ => panic!("Expected a type annotation after colon"),
                    },
                    _ => panic!("Expected a colon after value name"),
                };
                match tokens.next() {
                    Some(Token::Equal) => {}
                    _ => panic!("Expected an equal sign after type annotation"),
                }
                let value = parse_expression(&mut tokens);
                statements.push(AbstractSyntaxTree::Const {
                    name: name.clone(),
                    type_annot: type_annotation.clone(),
                    value: Box::new(value),
                });
            }
            Token::Fn => {
                let name = match tokens.next() {
                    Some(Token::Name { name }) => name,
                    _ => panic!("Expected a name after fn"),
                };
                let mut args = Vec::new();
                match tokens.next() {
                    Some(Token::LeftParen) => {}
                    _ => panic!("Expected a left paren after fn"),
                }
                while let Some(token) = tokens.next() {
                    match token {
                        Token::RightParen => break,
                        Token::Name { name } => args.push(name),
                        _ => panic!("Expected a name or right paren after left paren"),
                    }
                }
                match tokens.next() {
                    Some(Token::LeftBrace) => {}
                    _ => panic!("Expected a left brace after fn args"),
                }
                let body = parse_fn_body(&mut tokens);
                statements.push(AbstractSyntaxTree::Fn {
                    name: name.clone(),
                    // args,
                    body: Box::new(body),
                });
            }
            _ => {
                panic!("Unexpected token")
            }
        }
    }
    AbstractSyntaxTree::Block { statements }
}

fn parse_fn_body(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Vec<AbstractSyntaxTree> {
    let mut statements = Vec::new();
    while let Some(token) = tokens.next() {
        match token {
            Token::RightBrace => break,
            Token::Name { name } => match tokens.next() {
                Some(Token::LeftParen) => {
                    let arg = parse_expression(tokens);
                    match tokens.next() {
                        Some(Token::RightParen) => {}
                        _ => panic!("Expected a right paren after function args"),
                    }
                    statements.push(AbstractSyntaxTree::Call {
                        name: name.clone(),
                        args: vec![arg],
                    });
                }
                _ => panic!("Expected a left paren after name"),
            },
            Token::Let => {
                let name = match tokens.next() {
                    Some(Token::Name { name }) => name,
                    _ => panic!("Expected a name after let"),
                };
                let type_annotation: String = match tokens.next() {
                    Some(Token::Colon) => match tokens.next() {
                        Some(Token::UpName { name }) => name.clone(),
                        _ => panic!("Expected a type annotation after colon"),
                    },
                    _ => panic!("Expected a colon after value name"),
                };
                match tokens.next() {
                    Some(Token::Equal) => {}
                    _ => panic!("Expected an equal sign after type annotation"),
                }
                let value = parse_expression(tokens);
                statements.push(AbstractSyntaxTree::Let {
                    name: name.clone(),
                    value: Box::new(value),
                    type_annot: type_annotation.clone(),
                });
            }
            _ => {
                println!("{:?}", token);
                panic!("Unexpected token")
            }
        }
    }
    statements
}

fn parse_expression(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> AbstractSyntaxTree {
    let mut left = match tokens.next() {
        Some(Token::Int { value }) => AbstractSyntaxTree::Int {
            value: value.parse().unwrap(),
        },
        Some(Token::String { value }) => AbstractSyntaxTree::String {
            value: value.clone(),
        },
        Some(Token::Not) => {
            let value = parse_expression(tokens);
            AbstractSyntaxTree::Not {
                value: Box::new(value),
            }
        }
        Some(Token::Name { name }) => AbstractSyntaxTree::Name { name: name.clone() },
        Some(Token::UpName { name }) => AbstractSyntaxTree::UpName { name: name.clone() },
        Some(Token::LeftParen) => parse_expression(tokens),
        Some(Token::LeftBrace) => {
            let mut statements = Vec::new();
            while let Some(token) = tokens.next() {
                match token {
                    Token::RightBrace => break,
                    _ => {
                        tokens.next();
                        let statement = parse_expression(tokens);
                        statements.push(statement);
                    }
                }
            }
            AbstractSyntaxTree::Block { statements }
        }
        None => panic!("Unexpected end of input"),
        Some(token) => {
            panic!("Unexpected token: {:?}", token)
        }
    };
    while let Some(token) = tokens.peek() {
        match token {
            Token::Plus => {
                tokens.next();
                let right = parse_expression(tokens);
                left = AbstractSyntaxTree::Plus {
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
            Token::Minus => {
                tokens.next();
                let right = parse_expression(tokens);
                left = AbstractSyntaxTree::Minus {
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
            Token::LtGt => {
                tokens.next();
                let right = parse_expression(tokens);
                left = AbstractSyntaxTree::LtGt {
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
            Token::And => {
                tokens.next();
                let right = parse_expression(tokens);
                left = AbstractSyntaxTree::And {
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
            Token::Or => {
                tokens.next();
                let right = parse_expression(tokens);
                left = AbstractSyntaxTree::Or {
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
            Token::Not => {
                tokens.next();
                let value = parse_expression(tokens);
                left = AbstractSyntaxTree::Not {
                    value: Box::new(value),
                };
            }
            _ => break,
        }
    }
    left
}

fn lex(source: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut source = source.chars().peekable();

    while let Some(c) = source.next() {
        match c {
            // Skip whitespace
            ' ' | '\t' | '\r' | '\n' => continue,
            // Groupings
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            // Int Operators
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            // String Operators
            '<' => match source.peek() {
                Some(&'>') => {
                    source.next();
                    tokens.push(Token::LtGt);
                }
                _ => tokens.push(Token::UnexpectedGrapheme(c.to_string())),
            },
            // Boolean Operators
            '&' => match source.peek() {
                Some(&'&') => {
                    source.next();
                    tokens.push(Token::And);
                }
                _ => tokens.push(Token::UnexpectedGrapheme(c.to_string())),
            },
            '|' => match source.peek() {
                Some(&'|') => {
                    source.next();
                    tokens.push(Token::Or);
                }
                _ => tokens.push(Token::UnexpectedGrapheme(c.to_string())),
            },
            '!' => tokens.push(Token::Not),
            // Other Punctuation
            ':' => tokens.push(Token::Colon),
            '=' => tokens.push(Token::Equal),
            // Keywords
            'a'..='z' | 'A'..='Z' => {
                let mut name = String::new();
                name.push(c);
                while let Some(&c) = source.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            name.push(c);
                            source.next();
                        }
                        _ => break,
                    }
                }
                match name.as_str() {
                    "const" => tokens.push(Token::Const),
                    "fn" => tokens.push(Token::Fn),
                    "let" => tokens.push(Token::Let),
                    _ => {
                        if let Some(first_char) = name.chars().next() {
                            if first_char.is_uppercase() {
                                tokens.push(Token::UpName { name })
                            } else if first_char.is_lowercase() {
                                tokens.push(Token::Name { name })
                            } else {
                                tokens.push(Token::UnexpectedGrapheme(name));
                            }
                        } else {
                            tokens.push(Token::UnexpectedGrapheme(name));
                        }
                    }
                }
            }
            // Int
            '0'..='9' => {
                let mut value = String::new();
                value.push(c);
                while let Some(&c) = source.peek() {
                    match c {
                        '0'..='9' => {
                            value.push(c);
                            source.next();
                        }
                        _ => break,
                    }
                }
                tokens.push(Token::Int { value });
            }
            // String
            '"' => {
                let mut value = String::new();
                while let Some(c) = source.next() {
                    match c {
                        '"' => {
                            tokens.push(Token::String { value });
                            break;
                        }
                        '\n' => {
                            tokens.push(Token::UnterminatedString(value));
                            break;
                        }
                        _ => value.push(c),
                    }
                }
            }

            // Invalid
            _ => tokens.push(Token::UnexpectedGrapheme(c.to_string())),
        }
    }
    tokens
}

// this should be written as an enum with the arguements for each opcode type explicitly defined
#[derive(Debug, Clone)]
enum OpCode {
    And {
        arg1: usize,
        arg2: usize,
        arg3: usize,
    },
    Or {
        arg1: usize,
        arg2: usize,
        arg3: usize,
    },
    Not {
        value: usize,
    },
    Add {
        arg1: usize,
        arg2: usize,
        arg3: usize,
    },
    Sub {
        arg1: usize,
        arg2: usize,
        arg3: usize,
    },
    Load {
        arg1: usize,
        arg2: usize,
    },
    LoadImm {
        arg1: usize,
        arg2: usize,
    },
    Move {
        arg1: usize,
        arg2: usize,
    },
    Store {
        arg1: usize,
        arg2: usize,
    },
    StoreImm {
        arg1: usize,
        arg2: usize,
    },
    Print {
        arg1: usize,
    },
    Halt,
}

#[derive(Debug)]
struct SymbolTable {
    variables: Vec<Variable>,
    functions: Vec<Function>,
    constants: Vec<Constant>,
}
#[derive(Debug)]
struct Constant {
    name: String,
    type_annot: String,
    value: String,
}
#[derive(Debug)]
struct Variable {
    name: String,
    type_annot: String,
}
#[derive(Debug)]
struct Function {
    name: String,
}

#[derive(Debug, Clone)]
enum AbstractSyntaxTree {
    Const {
        name: String,
        type_annot: String,
        value: Box<AbstractSyntaxTree>,
    },
    Let {
        name: String,
        type_annot: String,
        value: Box<AbstractSyntaxTree>,
    },
    Int {
        value: usize,
    },
    String {
        value: String,
    },
    Name {
        name: String,
    },
    UpName {
        name: String,
    },
    Plus {
        left: Box<AbstractSyntaxTree>,
        right: Box<AbstractSyntaxTree>,
    },
    Minus {
        left: Box<AbstractSyntaxTree>,
        right: Box<AbstractSyntaxTree>,
    },
    LtGt {
        left: Box<AbstractSyntaxTree>,
        right: Box<AbstractSyntaxTree>,
    },
    And {
        left: Box<AbstractSyntaxTree>,
        right: Box<AbstractSyntaxTree>,
    },
    Or {
        left: Box<AbstractSyntaxTree>,
        right: Box<AbstractSyntaxTree>,
    },
    Not {
        value: Box<AbstractSyntaxTree>,
    },
    Fn {
        name: String,
        // args: Vec<String>,
        body: Box<Vec<AbstractSyntaxTree>>,
    },
    Call {
        name: String,
        args: Vec<AbstractSyntaxTree>,
    },
    Block {
        statements: Vec<AbstractSyntaxTree>,
    },
}

#[derive(Debug)]
enum Token {
    Name { name: String },
    UpName { name: String },
    Int { value: String },
    String { value: String },
    // Groupings
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    // Int Operators
    Plus,
    Minus,
    // String Operators
    LtGt, // '<>'
    // Boolean Operators
    And, // &&
    Or,  // ||
    Not, // !
    // Other Punctuation
    Colon,
    Equal,
    // Keywords (alphabetically):
    Const,
    Fn,
    Let,

    // Invalid code tokens
    UnterminatedString(String),
    UnexpectedGrapheme(String),
}
