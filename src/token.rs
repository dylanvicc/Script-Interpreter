#[derive(Debug)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParenthesis,
    RightParenthesis,
    WaitKeyword,
    PrintKeyword,
    LetKeyword,
    Identifier(String),
    Equal,
    Semicolon
}