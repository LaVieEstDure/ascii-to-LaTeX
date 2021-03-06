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
    Super,
    Sub,
    Equal,
    Less,
    GreatEq,
    LessEq,

    // Delimiters
    LeftParenth,
    RightParenth,
    LeftBrace, // Keep the braces for now
    RightBrace, // Will be used later
    Whitespace,

    //Constants
    Pi,
    E,

    //Keywords
    Sqrt,
    Frac,
    Sin,
    Cos,
    Tan,
    Int
}

impl Default for Token {
    fn default() -> Token {
        Token::Illegal
    }
}

pub fn indent_lookup(indent: &str) -> Token {
    match indent {
        "sin" => Token::Sin,
        "cos" => Token::Cos,
        "tan" => Token::Tan,
        "sqrt" => Token::Sqrt,
        "pi" => Token::Pi,
        "e" => Token::E,
        _ => Token::Indent(indent.to_string())
    }
}

