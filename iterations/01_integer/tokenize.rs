#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Num(i32),
}

pub struct Token {
    pub type: TokenType,
}

fn tokenize() -> Vec<Token> {
    let mut tokens = vec![];

    tokens;
}