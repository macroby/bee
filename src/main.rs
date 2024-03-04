use std::fs;

fn main() {
    let contents = fs::read_to_string("test.bee").expect("Should have been able to read the file");
    let tokens = lex(contents.clone());
    for token in tokens {
        println!("{:?}", token);
    }
    println!("With text:\n{contents}");
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
