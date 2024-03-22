use core::panic;
use std::{collections::HashMap, fs};

mod ast;
mod code_gen;
mod lex;
mod opcode;
mod parse;
mod token;

use ast::AbstractSyntaxTree;
use code_gen::{code_gen, Constant, Function, SymbolTable, Variable};
use lex::lex;
use opcode::OpCode;
use parse::parse;

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

#[derive(Debug, Clone)]
enum Value {
    Int(usize),
    String(Box<str>),
}

impl Operation for Value {
    fn add(&self, other: &Self) -> Self {
        match self {
            Value::Int(value) => match other {
                Value::Int(other_value) => Value::Int(value + other_value),
                _ => panic!("Invalid operation"),
            },
            Value::String(_) => panic!("Invalid operation"),
        }
    }

    fn sub(&self, other: &Self) -> Self {
        match self {
            Value::Int(value) => match other {
                Value::Int(other_value) => Value::Int(value - other_value),
                _ => panic!("Invalid operation"),
            },
            Value::String(_) => panic!("Invalid operation"),
        }
    }

    fn and(&self, other: &Self) -> Self {
        match self {
            Value::Int(value) => match other {
                Value::Int(other_value) => Value::Int(value & other_value),
                _ => panic!("Invalid operation"),
            },
            Value::String(_) => panic!("Invalid operation"),
        }
    }

    fn or(&self, other: &Self) -> Self {
        match self {
            Value::Int(value) => match other {
                Value::Int(other_value) => Value::Int(value | other_value),
                _ => panic!("Invalid operation"),
            },
            Value::String(_) => panic!("Invalid operation"),
        }
    }

    fn not(&self) -> Self {
        match self {
            Value::Int(value) => Value::Int(value ^ 1),
            Value::String(_) => panic!("Invalid operation"),
        }
    }

    fn concat(&self, other: &Self) -> Self {
        match self {
            Value::String(value) => match other {
                Value::String(other_value) => {
                    Value::String(format!("{}{}", value, other_value).into())
                }
                _ => panic!("Invalid operation"),
            },
            Value::Int(_) => panic!("Invalid operation"),
        }
    }
}

pub trait Operation {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn and(&self, other: &Self) -> Self;
    fn or(&self, other: &Self) -> Self;
    fn not(&self) -> Self;
    fn concat(&self, other: &Self) -> Self;
}

const INIT: Value = Value::Int(0);
fn interpret(op_codes: Vec<OpCode>) {
    let mut registers: [Value; 32] = [INIT; 32];

    // Does this need to be a hash map? since everything is index by usize
    // we should be able to use a vector
    let mut variables: HashMap<usize, Value> = HashMap::new();
    let mut pc = 0;
    loop {
        let op_code = &op_codes[pc];
        match op_code {
            OpCode::Add { arg1, arg2, arg3 } => {
                registers[*arg1] = registers[*arg2].add(&registers[*arg3]);
            }
            OpCode::Sub { arg1, arg2, arg3 } => {
                registers[*arg1] = registers[*arg2].sub(&registers[*arg3]);
            }
            OpCode::And { arg1, arg2, arg3 } => {
                registers[*arg1] = registers[*arg2].and(&registers[*arg3]);
            }
            OpCode::Or { arg1, arg2, arg3 } => {
                registers[*arg1] = registers[*arg2].or(&registers[*arg3]);
            }
            OpCode::Not { value } => {
                registers[*value] = registers[*value].not();
            }
            OpCode::Concat { arg1, arg2, arg3 } => {
                registers[*arg1] = registers[*arg2].concat(&registers[*arg3]);
            }
            OpCode::Load { arg1, arg2 } => {
                registers[*arg1] = variables.get(&arg2).unwrap().clone();
            }
            OpCode::LoadIntConst { arg1, arg2 } => {
                registers[*arg1] = Value::Int(*arg2);
            }
            OpCode::LoadStringConst { arg1, arg2 } => {
                registers[*arg1] = Value::String(arg2.clone());
            }
            OpCode::Move { arg1, arg2 } => {
                registers[*arg1] = registers[*arg2].to_owned();
            }
            OpCode::Store { arg1, arg2 } => {
                variables.insert(*arg1, registers[*arg2].to_owned());
            }
            OpCode::StoreIntConst { arg1, arg2 } => {
                variables.insert(*arg1, Value::Int(*arg2));
            }
            OpCode::StoreStringConst { arg1, arg2 } => {
                variables.insert(*arg1, Value::String(arg2.clone()));
            }
            OpCode::Print { arg1 } => {
                println!("{:?}", registers[*arg1]);
            }
            OpCode::Halt => {
                break;
            }
        }
        pc += 1;
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
