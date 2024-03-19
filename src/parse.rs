use crate::ast::AbstractSyntaxTree;
use crate::token::Token;

pub fn parse(tokens: Vec<Token>) -> AbstractSyntaxTree {
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
