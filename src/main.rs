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
    Float(f64),
    String(Box<str>),
}

impl Operation for Value {
    fn add(&self, other: &Self) -> Self {
        match self {
            Value::Int(value) => match other {
                Value::Int(other_value) => Value::Int(value + other_value),
                _ => panic!("Invalid operation"),
            },
            Value::Float(value) => match other {
                Value::Float(other_value) => Value::Float(value + other_value),
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
            Value::Float(value) => match other {
                Value::Float(other_value) => Value::Float(value - other_value),
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
            Value::Float(_) => panic!("Invalid operation"),
            Value::String(_) => panic!("Invalid operation"),
        }
    }

    fn or(&self, other: &Self) -> Self {
        match self {
            Value::Int(value) => match other {
                Value::Int(other_value) => Value::Int(value | other_value),
                _ => panic!("Invalid operation"),
            },
            Value::Float(_) => panic!("Invalid operation"),
            Value::String(_) => panic!("Invalid operation"),
        }
    }

    fn not(&self) -> Self {
        match self {
            Value::Int(value) => Value::Int(value ^ 1),
            Value::Float(_) => panic!("Invalid operation"),
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
            Value::Float(_) => panic!("Invalid operation"),
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
            OpCode::LoadFloatConst { arg1, arg2 } => {
                registers[*arg1] = Value::Float(*arg2);
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
            OpCode::StoreFloatConst { arg1, arg2 } => {
                variables.insert(*arg1, Value::Float(*arg2));
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
                AbstractSyntaxTree::Float { value } => value.to_string(),
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
                AbstractSyntaxTree::Float { .. } => {
                    if type_annot != "Float" {
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
                    "add_float" | "sub_float" => {
                        for arg in args {
                            match arg {
                                AbstractSyntaxTree::Float { .. } => {
                                    if type_annot != "Float" {
                                        panic!("Type mismatch");
                                    }
                                }
                                AbstractSyntaxTree::Name { name } => {
                                    match symbol_table.constants.iter().find(|v| v.name == *name) {
                                        Some(v) => {
                                            if v.type_annot != "Float" {
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
                                                    if v.type_annot != "Float" {
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
    fn int_const_sub_add_print() {
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

    #[test]
    fn string_const_concat_print() {
        let contents = r#"
            const ctest: String = "Hello"
    
            fn main() {
                let one: String = "World"
                let one_again: String = one
                let five: String = concat(one, " ")
                let itest: String = concat(five, ctest)
                print_string(itest)
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

    #[test]
    fn bool_const_and_or_not_print() {
        let contents = r#"
            const ctest: Bool = True
    
            fn main() {
                let one: Bool = False
                let one_again: Bool = one
                let five: Bool = and(one, True)
                let itest: Bool = or(five, ctest)
                let not_itest: Bool = not(itest)
                print_bool(not_itest)
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

    #[test]
    fn float_add_sub_print() {
        let contents = r#"
            const ctest: Float = 5.0
    
            fn main() {
                let one: Float = 1.0
                let one_again: Float = one
                let five: Float = add_float(one, 4.0)
                let itest: Float = sub_float(five, 4.0)
                let ctest_add: Float = add_float(1.0, ctest)
                print_float(itest)
                print_float(ctest_add)
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
