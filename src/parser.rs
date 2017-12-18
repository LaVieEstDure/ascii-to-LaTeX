use token::Token;
use std::iter::Peekable;
use lexer;

#[derive(Debug,PartialEq)] 
pub enum Expression {
    Number(String),
    Unary(Operation, Box<Expression>),
    BinaryExpr(Box<Expression>, Operation, Box<Expression>)
}

#[derive(Debug,PartialEq)] 
pub enum Operation {
    Fraction,
    Function,
    Plus,
    Minus
}

pub struct Parser<'a> {
    equation: lexer::Lexer<'a>
}

pub fn get_priority(token: &Token) -> u32 {
    match token {
        &Token::Multiply | &Token::Divide => 3,
        &Token::Plus | &Token::Minus => 2,
        _ => 1
    }
}

pub fn parse<I> (mut eq: Peekable<I>) -> Expression 
    where I: Iterator<Item=Token> {
    let first = match eq.next() {
        Some(Token::Number(n)) => Expression::Number(n),
        _ => Expression::Number(String::from(""))
    };
    match eq.next() {
        Some(Token::Plus) => Expression::BinaryExpr(Box::new(first), Operation::Plus, Box::new(Expression::Number("2".to_string()))),
        _ => Expression::Number("1".to_string())
    }
}