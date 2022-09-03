#[derive(Debug)]
pub struct TokenError {
    pub(crate) message: String,
    pub(crate) path: String,
    pub(crate) position: (u16, u16)
}

#[derive(Debug)]
pub enum TokenType {
    Number(String),
    String(String),
    Char(String),
    Boolean(String),
    Operator(String),
    Keyword(String),
    Identifier(String),
    Error(TokenError),
    Other(String),
    Space,
    End
}

#[derive(Debug)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) tokens: Vec<Token>
}