use core::panic;
use std::collections::HashMap;

mod ast;
mod code_gen;
mod lex;
mod opcode;
mod parse;
mod token;

use ast::{AbstractSyntaxTree, Document};
use code_gen::{code_gen_document, Constant, Function, SymbolTable, Variable};
use lex::lex;
use opcode::OpCode;
use parse::parse;

fn main() {
    let contents = r#"
    const ctest: Integer = 5
    
    fn main() {
      let one: Integer = 1
      let one_again: Integer = one
      let five: Integer = add(one, 4)
      let itest: Integer = sub(five, 4)
      let ctest_add: Integer = add(1, ctest)
      print_integer(itest)
      print_integer(ctest_add)
    }"#;
    let tokens = lex(contents.to_string());
    let document = parse(tokens);
    let mut symbol_table = SymbolTable {
        functions: Vec::new(),
        constants: Vec::new(),
    };
    analyze_document(document.clone(), &mut symbol_table);
    println!("Symbol Table:");
    println!("{:?}", symbol_table);

    let mut op_codes = Vec::new();
    code_gen_document(document, &mut symbol_table, &mut op_codes);

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
pub fn interpret(op_codes: Vec<OpCode>) {
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

pub fn analyze_document(document: Document, symbol_table: &mut SymbolTable) {
    for constant in document.constants {
        symbol_table.constants.push(Constant {
            name: constant.name.clone(),
            type_annot: constant.type_annot.clone(),
            value: match constant.value {
                AbstractSyntaxTree::Int { value } => value.to_string(),
                AbstractSyntaxTree::String { value } => value.clone(),
                AbstractSyntaxTree::UpName { name } => {
                    if name == "True" {
                        "1".to_owned()
                    } else if name == "False" {
                        "0".to_owned()
                    } else {
                        panic!("Invalid value")
                    }
                }
                _ => panic!("Invalid value"),
            },
        });
    }
    for function in document.functions {
        symbol_table.functions.push(Function {
            name: function.name.clone(),
            variables: Vec::new(),
        });
        for statement in function.body {
            analyze(statement, symbol_table);
        }
    }
}

pub fn analyze(ast: AbstractSyntaxTree, symbol_table: &mut SymbolTable) {
    match ast {
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
                    match symbol_table.constants.iter().find(|v| v.name == name) {
                        Some(v) => {
                            if v.type_annot != type_annot {
                                panic!("Type mismatch");
                            }
                        }
                        None => {
                            let variable = symbol_table
                                .functions
                                .last()
                                .unwrap()
                                .variables
                                .iter()
                                .find(|v| v.name == name);
                            match variable {
                                Some(v) => {
                                    if v.type_annot != type_annot {
                                        panic!("Type mismatch");
                                    }
                                }
                                None => panic!("Variable not found"),
                            }
                        }
                    }
                }
                AbstractSyntaxTree::Call { name, args } => match name.as_str() {
                    "add" | "sub" => {
                        for arg in args {
                            match arg {
                                AbstractSyntaxTree::Int { .. } => {
                                    if type_annot != "Integer" {
                                        panic!("Type mismatch");
                                    }
                                }
                                AbstractSyntaxTree::Name { name } => {
                                    match symbol_table.constants.iter().find(|v| v.name == *name) {
                                        Some(v) => {
                                            if v.type_annot != "Integer" {
                                                panic!("Type mismatch");
                                            }
                                        }
                                        None => {
                                            let variable = symbol_table
                                                .functions
                                                .last()
                                                .unwrap()
                                                .variables
                                                .iter()
                                                .find(|v| v.name == *name);
                                            match variable {
                                                Some(v) => {
                                                    if v.type_annot != "Integer" {
                                                        panic!("Type mismatch");
                                                    }
                                                }
                                                None => panic!("Variable not found"),
                                            }
                                        }
                                    }
                                }
                                _ => panic!("Invalid value"),
                            }
                        }
                    }
                    "and" | "or" => {
                        for arg in args {
                            match arg {
                                AbstractSyntaxTree::UpName { .. } => {
                                    if type_annot != "Bool" {
                                        panic!("Type mismatch");
                                    }
                                }
                                AbstractSyntaxTree::Name { name } => {
                                    match symbol_table.constants.iter().find(|v| v.name == *name) {
                                        Some(v) => {
                                            if v.type_annot != "Bool" {
                                                panic!("Type mismatch");
                                            }
                                        }
                                        None => {
                                            let variable = symbol_table
                                                .functions
                                                .last()
                                                .unwrap()
                                                .variables
                                                .iter()
                                                .find(|v| v.name == *name);
                                            match variable {
                                                Some(v) => {
                                                    if v.type_annot != "Bool" {
                                                        panic!("Type mismatch");
                                                    }
                                                }
                                                None => panic!("Variable not found"),
                                            }
                                        }
                                    }
                                }
                                _ => panic!("Invalid value"),
                            }
                        }
                    }
                    "not" => {
                        for arg in args {
                            match arg {
                                AbstractSyntaxTree::UpName { .. } => {
                                    if type_annot != "Bool" {
                                        panic!("Type mismatch");
                                    }
                                }
                                AbstractSyntaxTree::Name { name } => {
                                    match symbol_table.constants.iter().find(|v| v.name == *name) {
                                        Some(v) => {
                                            if v.type_annot != "Bool" {
                                                panic!("Type mismatch");
                                            }
                                        }
                                        None => {
                                            let variable = symbol_table
                                                .functions
                                                .last()
                                                .unwrap()
                                                .variables
                                                .iter()
                                                .find(|v| v.name == *name);
                                            match variable {
                                                Some(v) => {
                                                    if v.type_annot != "Bool" {
                                                        panic!("Type mismatch");
                                                    }
                                                }
                                                None => panic!("Variable not found"),
                                            }
                                        }
                                    }
                                }
                                _ => panic!("Invalid value"),
                            }
                        }
                    }
                    "concat" => {
                        for arg in args {
                            match arg {
                                AbstractSyntaxTree::String { .. } => {
                                    if type_annot != "String" {
                                        panic!("Type mismatch");
                                    }
                                }
                                AbstractSyntaxTree::Name { name } => {
                                    match symbol_table.constants.iter().find(|v| v.name == *name) {
                                        Some(v) => {
                                            if v.type_annot != "String" {
                                                panic!("Type mismatch");
                                            }
                                        }
                                        None => {
                                            let variable = symbol_table
                                                .functions
                                                .last()
                                                .unwrap()
                                                .variables
                                                .iter()
                                                .find(|v| v.name == *name);
                                            match variable {
                                                Some(v) => {
                                                    if v.type_annot != "String" {
                                                        panic!("Type mismatch");
                                                    }
                                                }
                                                None => panic!("Variable not found"),
                                            }
                                        }
                                    }
                                }
                                _ => panic!("Invalid value"),
                            }
                        }
                    }
                    _ => {
                        let function = symbol_table.functions.iter().find(|f| f.name == name);
                        match function {
                            Some(f) => {
                                for (i, arg) in args.iter().enumerate() {
                                    match arg {
                                        AbstractSyntaxTree::Int { .. } => {
                                            if f.variables[i].type_annot != "Integer" {
                                                panic!("Type mismatch");
                                            }
                                        }
                                        AbstractSyntaxTree::String { .. } => {
                                            if f.variables[i].type_annot != "String" {
                                                panic!("Type mismatch");
                                            }
                                        }
                                        AbstractSyntaxTree::UpName { .. } => {
                                            if f.variables[i].type_annot != "Bool" {
                                                panic!("Type mismatch");
                                            }
                                        }
                                        AbstractSyntaxTree::Name { name } => match symbol_table
                                            .constants
                                            .iter()
                                            .find(|v| v.name == *name)
                                        {
                                            Some(v) => {
                                                if v.type_annot != f.variables[i].type_annot {
                                                    panic!("Type mismatch");
                                                }
                                            }
                                            None => {
                                                let variable = symbol_table
                                                    .functions
                                                    .last()
                                                    .unwrap()
                                                    .variables
                                                    .iter()
                                                    .find(|v| v.name == *name);
                                                match variable {
                                                    Some(v) => {
                                                        if v.type_annot != f.variables[i].type_annot
                                                        {
                                                            panic!("Type mismatch");
                                                        }
                                                    }
                                                    None => panic!("Variable not found"),
                                                }
                                            }
                                        },
                                        AbstractSyntaxTree::Call { .. } => {
                                            panic!("Cant handle nested function calls")
                                        }
                                        _ => panic!("Invalid value"),
                                    }
                                }
                            }
                            None => panic!("Function not found"),
                        }
                    }
                },
                some_value => {
                    println!("{:?}", some_value);
                    panic!("Invalid value")
                }
            }

            let mut new_function = symbol_table.functions.pop().expect("Function not found");
            new_function.variables.push(Variable {
                name: name.clone(),
                type_annot: type_annot.clone(),
            });

            symbol_table.functions.push(new_function);
        }
        AbstractSyntaxTree::Block { statements } => {
            for statement in statements {
                analyze(statement, symbol_table);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use code_gen::code_gen_document;
    use lex::lex;
    use parse::parse;
    #[test]
    fn it_works() {
        let contents = r#"
            const ctest: Integer = 5
    
            fn main() {
                let one: Integer = 1
                let one_again: Integer = one
                let five: Integer = add(one, 4)
                let itest: Integer = sub(five, 4)
                let ctest_add: Integer = add(1, ctest)
                print_integer(itest)
                print_integer(ctest_add)
            }"#;
        let tokens = lex(contents.to_string());
        let document = parse(tokens);
        println!("Document:");
        println!("{:?}", document);
        let mut symbol_table = SymbolTable {
            functions: Vec::new(),
            constants: Vec::new(),
        };
        analyze_document(document.clone(), &mut symbol_table);
        println!("Symbol Table:");
        println!("{:?}", symbol_table);

        let mut op_codes = Vec::new();
        code_gen_document(document, &mut symbol_table, &mut op_codes);

        println!("Codegen Output:");
        for op_code in op_codes.clone() {
            println!("{:?}", op_code);
        }

        println!("Interpretation:");
        interpret(op_codes);
    }
}
