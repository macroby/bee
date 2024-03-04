use core::panic;
use std::fs;

fn main() {
    let contents = fs::read_to_string("test.bee").expect("Should have been able to read the file");
    let tokens = lex(contents.clone());
    let ast = parse(tokens);
    println!("{:?}", ast);
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

#[derive(Debug)]
enum AbstractSyntaxTree {
    // Statements
    Let {
        name: String,
        type_annot: String,
        value: Box<AbstractSyntaxTree>,
    },
    // Expressions
    Int {
        value: i64,
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
