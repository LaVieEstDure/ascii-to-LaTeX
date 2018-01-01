#[derive(Debug, PartialEq)]
pub enum Token{
    // meta
    Illegal,
    EOF,

    Indent(String),
    Integer(String),
    Number(String),

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

    //Constants
    Pi,
    E,

    //Keywords
    Root,
    Frac,
    Sin,
    Cos,
    Tan
}

impl Default for Token {
    fn default() -> Token {
        Token::Illegal
    }
}

pub fn indent_lookup(indent: &str) -> Token {
    match indent {
        "sin" => Token::Sin,
        "sqrt" => Token::Root,
        "pi" => Token::Pi,
        "e" => Token::E,
        _ => Token::Indent(indent.to_string())
    }
}

