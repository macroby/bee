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
        match op_code.name {
            OpCodeName::Add => {
                let arg1 = op_code.args[0];
                let arg2 = op_code.args[1];
                let arg3 = op_code.args[2];
                registers[arg1] = registers[arg2] + registers[arg3];
            }
            OpCodeName::Sub => {
                let arg1 = op_code.args[0];
                let arg2 = op_code.args[1];
                let arg3 = op_code.args[2];
                registers[arg1] = registers[arg2] - registers[arg3];
            }
            OpCodeName::Load => {
                let arg1 = op_code.args[0];
                let arg2 = &op_code.args[1];
                registers[arg1] = *variables.get(arg2).unwrap();
            }
            OpCodeName::LoadImm => {
                let arg1 = op_code.args[0];
                let arg2 = op_code.args[1];
                registers[arg1] = arg2;
            }
            OpCodeName::Move => {
                let arg1 = op_code.args[0];
                let arg2 = op_code.args[1];
                registers[arg1] = registers[arg2];
            }
            OpCodeName::Store => {
                let arg1 = &op_code.args[0];
                let arg2 = op_code.args[1];
                variables.insert(*arg1, registers[arg2]);
            }
            OpCodeName::StoreImm => {
                let arg1 = &op_code.args[0];
                let arg2 = op_code.args[1];
                variables.insert(*arg1, arg2);
            }
            OpCodeName::Print => {
                let arg1 = op_code.args[0];
                println!("{}", registers[arg1]);
            }
            OpCodeName::Printi => {
                let arg1 = op_code.args[0];
                println!("{}", registers[arg1]);
            }
            OpCodeName::Halt => {
                break;
            }
        }
        pc += 1;
    }
}

fn code_gen(ast: AbstractSyntaxTree, symbol_table: &mut SymbolTable, op_codes: &mut Vec<OpCode>) {
    match ast {
        AbstractSyntaxTree::Let {
            name,
            type_annot: _,
            value,
        } => match *value {
            AbstractSyntaxTree::Name { name: var_name } => {
                let var_index = symbol_table
                    .variables
                    .iter()
                    .position(|v| v.name == var_name)
                    .unwrap();
                let target_index = symbol_table
                    .variables
                    .iter()
                    .position(|v| v.name == name)
                    .unwrap();
                op_codes.push(OpCode {
                    name: OpCodeName::Load,
                    args: vec![1, var_index],
                });
                op_codes.push(OpCode {
                    name: OpCodeName::Store,
                    args: vec![target_index, 1],
                });
            }
            AbstractSyntaxTree::UpName { name: upname } => match upname.as_str() {
                "True" => {
                    let target_index = symbol_table
                        .variables
                        .iter()
                        .position(|v| v.name == name)
                        .unwrap();
                    op_codes.push(OpCode {
                        name: OpCodeName::StoreImm,
                        args: vec![target_index, 1],
                    });
                }
                "False" => {
                    let target_index = symbol_table
                        .variables
                        .iter()
                        .position(|v| v.name == name)
                        .unwrap();
                    op_codes.push(OpCode {
                        name: OpCodeName::StoreImm,
                        args: vec![target_index, 0],
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
                op_codes.push(OpCode {
                    name: OpCodeName::StoreImm,
                    args: vec![target_index, value as usize],
                });
            }
            AbstractSyntaxTree::Minus { left, right } => {
                match *left {
                    AbstractSyntaxTree::Int { value } => {
                        op_codes.push(OpCode {
                            name: OpCodeName::LoadImm,
                            args: vec![1, value as usize],
                        });
                    }
                    AbstractSyntaxTree::Name { name } => {
                        let var_index = symbol_table
                            .variables
                            .iter()
                            .position(|v| v.name == name)
                            .unwrap();
                        op_codes.push(OpCode {
                            name: OpCodeName::Load,
                            args: vec![1, var_index],
                        });
                    }
                    _ => panic!("Invalid value"),
                }

                match *right {
                    AbstractSyntaxTree::Int { value } => {
                        op_codes.push(OpCode {
                            name: OpCodeName::LoadImm,
                            args: vec![2, value as usize],
                        });
                    }
                    AbstractSyntaxTree::Name { name } => {
                        let var_index = symbol_table
                            .variables
                            .iter()
                            .position(|v| v.name == name)
                            .unwrap();
                        op_codes.push(OpCode {
                            name: OpCodeName::Load,
                            args: vec![2, var_index],
                        });
                    }
                    _ => panic!("Invalid value"),
                }

                op_codes.push(OpCode {
                    name: OpCodeName::Sub,
                    args: vec![3, 1, 2],
                });

                let target_index = symbol_table
                    .variables
                    .iter()
                    .position(|v| v.name == name)
                    .unwrap();

                op_codes.push(OpCode {
                    name: OpCodeName::Store,
                    args: vec![target_index, 3],
                });
            }
            AbstractSyntaxTree::Plus { left, right } => {
                match *left {
                    AbstractSyntaxTree::Int { value } => {
                        op_codes.push(OpCode {
                            name: OpCodeName::LoadImm,
                            args: vec![1, value as usize],
                        });
                    }
                    AbstractSyntaxTree::Name { name } => {
                        let var_index = symbol_table
                            .variables
                            .iter()
                            .position(|v| v.name == name)
                            .unwrap();
                        op_codes.push(OpCode {
                            name: OpCodeName::Load,
                            args: vec![1, var_index],
                        });
                    }
                    _ => panic!("Invalid value"),
                }

                match *right {
                    AbstractSyntaxTree::Int { value } => {
                        op_codes.push(OpCode {
                            name: OpCodeName::LoadImm,
                            args: vec![2, value as usize],
                        });
                    }
                    AbstractSyntaxTree::Name { name } => {
                        let var_index = symbol_table
                            .variables
                            .iter()
                            .position(|v| v.name == name)
                            .unwrap();
                        op_codes.push(OpCode {
                            name: OpCodeName::Load,
                            args: vec![2, var_index],
                        });
                    }
                    _ => panic!("Invalid value"),
                }

                op_codes.push(OpCode {
                    name: OpCodeName::Add,
                    args: vec![3, 1, 2],
                });

                let target_index = symbol_table
                    .variables
                    .iter()
                    .position(|v| v.name == name)
                    .unwrap();

                op_codes.push(OpCode {
                    name: OpCodeName::Store,
                    args: vec![target_index, 3],
                });
            }
            _ => panic!("Invalid value"),
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
            op_codes.push(OpCode {
                name: OpCodeName::Halt,
                args: vec![],
            });
        }
        AbstractSyntaxTree::Call { name, args } => {
            if name == "print_integer" {
                match args.first() {
                    Some(arg) => match arg {
                        AbstractSyntaxTree::Int { value } => {
                            op_codes.push(OpCode {
                                name: OpCodeName::Printi,
                                args: vec![*value as usize],
                            });
                        }
                        AbstractSyntaxTree::Name { name } => {
                            let var_index = symbol_table
                                .variables
                                .iter()
                                .position(|v| v.name == *name)
                                .unwrap();
                            op_codes.push(OpCode {
                                name: OpCodeName::Load,
                                args: vec![1, var_index],
                            });
                            op_codes.push(OpCode {
                                name: OpCodeName::Print,
                                args: vec![1],
                            });
                        }
                        _ => panic!("Invalid value"),
                    },
                    None => panic!("No args"),
                }
            }
        }
        _ => {
            panic!("Invalid code");
        }
    }
}

fn analyze(ast: AbstractSyntaxTree, symbol_table: &mut SymbolTable) {
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
        _ => {}
    }
}

fn parse(tokens: Vec<Token>) -> AbstractSyntaxTree {
    let mut tokens = tokens.iter().peekable();
    let mut statements = Vec::new();
    while let Some(token) = tokens.next() {
        match token {
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
        Some(_) => {
            panic!("Unexpected token")
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
struct OpCode {
    name: OpCodeName,
    args: Vec<usize>,
}

#[derive(Debug, Clone)]
enum OpCodeName {
    Add,
    Sub,
    Load,
    LoadImm,
    Move,
    Store,
    StoreImm,
    Print,
    Printi,
    Halt,
}

#[derive(Debug)]
struct SymbolTable {
    variables: Vec<Variable>,
    functions: Vec<Function>,
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
    // Statements
    Let {
        name: String,
        type_annot: String,
        value: Box<AbstractSyntaxTree>,
    },
    // Expressions
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
    // Operators
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
    // Functions
    Fn {
        name: String,
        // args: Vec<String>,
        body: Box<Vec<AbstractSyntaxTree>>,
    },
    Call {
        name: String,
        args: Vec<AbstractSyntaxTree>,
    },
    // Other
    Block {
        statements: Vec<AbstractSyntaxTree>,
    },
    // Invalid code tokens
    UnterminatedString(String),
    UnexpectedGrapheme(String),
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
    // Other Punctuation
    Colon,
    Equal,
    // Keywords (alphabetically):
    Fn,
    Let,

    // Invalid code tokens
    UnterminatedString(String),
    UnexpectedGrapheme(String),
}
