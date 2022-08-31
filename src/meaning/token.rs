#[derive(Debug)]
pub struct TokenError {
    pub(crate) message: String,
    pub(crate) path: String,
    pub(crate) position: (u16, u16)
}

#[derive(Debug)]
pub enum TokenType {
    Number(u128),
    String(String),
    Boolean(bool),
    Operator(String),
    Keyword(String),
    Macro(String),
    Identifier(String),
    Error(TokenError),
    Other(String),
    End
}

#[derive(Debug)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) tokens: Vec<Token>
}