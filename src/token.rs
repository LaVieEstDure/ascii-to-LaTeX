#[derive(Debug, PartialEq)]
pub enum Token{
    // meta
    Illegal,
    EOF,

    Indent(String),
    Integer(String),

    Plus,
    Minus,
    Multiply,   
    Divide,
    Greater,
    Equal,
    Less,
    GreatEq,
    LessEq,

    // Delimiters
    LeftParenth,
    RightParenth,
    LeftBrace,
    RightBrace,
    Whitespace,

    //Keywords
    Root,
    Frac
}

impl Default for Token {
    fn default() -> Token {
        Token::Illegal
    }
}

pub fn indent_lookup(indent: &str) -> Token {
    match indent {
        "/" => Token::Frac,
        _ => Token::Indent(indent.to_string())
    }
}

