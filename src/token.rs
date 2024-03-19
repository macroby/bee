#[derive(Debug)]
pub enum Token {
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
