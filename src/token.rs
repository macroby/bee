#[derive(Debug)]
pub enum Token {
    Name { name: String },
    UpName { name: String },
    Int { value: String },
    Float { value: String },
    String { value: String },
    // Groupings
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    Comma,
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
