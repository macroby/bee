use crate::ast::{AbstractSyntaxTree, Constant, Document, Function};
use crate::token::Token;

pub fn parse(tokens: Vec<Token>) -> Document {
    let mut tokens = tokens.iter().peekable();
    let mut functions = Vec::new();
    let mut constants = Vec::new();
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
                constants.push(Constant {
                    name: name.clone(),
                    type_annot: type_annotation.clone(),
                    value,
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
                functions.push(Function {
                    name: name.clone(),
                    body,
                });
            }
            _ => {
                panic!("Unexpected token")
            }
        }
    }
    Document {
        constants,
        functions,
    }
}

fn parse_expression(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> AbstractSyntaxTree {
    let ast = match tokens.next() {
        Some(Token::Int { value }) => AbstractSyntaxTree::Int {
            value: value.parse().unwrap(),
        },
        Some(Token::Float { value }) => AbstractSyntaxTree::Float {
            value: value.parse().unwrap(),
        },
        Some(Token::String { value }) => AbstractSyntaxTree::String {
            value: value.clone(),
        },
        Some(Token::Name { name }) => {
            //could be a function call or a variable
            match tokens.peek() {
                Some(Token::LeftParen) => {
                    tokens.next();
                    let mut args = Vec::new();
                    while let Some(token) = tokens.peek() {
                        match token {
                            Token::RightParen => {
                                tokens.next();
                                break;
                            }
                            _ => {
                                let arg = parse_expression(tokens);
                                args.push(arg);
                            }
                        }
                    }
                    AbstractSyntaxTree::Call {
                        name: name.clone(),
                        args,
                    }
                }
                _ => AbstractSyntaxTree::Name { name: name.clone() },
            }
        }
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
        Some(Token::Comma) => parse_expression(tokens),
        Some(token) => {
            println!("{:?}", tokens);
            panic!("Unexpected token: {:?}", token)
        }
        None => panic!("Unexpected end of input"),
    };
    ast
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
                Some(token) => {
                    println!("{:?}", statements);
                    panic!("Unexpected token: {:?}", token)
                }
                _ => {
                    println!("{:?}", statements);
                    panic!("Expected a left paren after name")
                }
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
                println!("{:?}", statements);
                println!("{:?}", token);
                panic!("Unexpected token")
            }
        }
    }
    statements
}
