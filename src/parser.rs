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
    match *token {
        Token::Multiply | Token::Divide => 3,
        Token::Plus | Token::Minus => 2,
        _ => 1
    }
}

pub fn is_operator(token: &Token) -> bool {
    match *token {
        Token::Plus
        | Token::Minus
        | Token::Multiply
        | Token::Divide => true,
        
        _ => false
    }
}

pub fn parse<I> (mut eq: Peekable<I>) -> Expression 
    where I: Iterator<Item=Token> {
    let mut first: Vec<Token> = vec!(eq.next().unwrap());

    while !is_operator((eq.peek().unwrap())){
        first.push(eq.next().unwrap())
    }

    println!("{:?}", first);
    Expression::Number("0".to_string())
}

pub fn parse_prefix_expr<I>(mut eq: Peekable<I>) -> Expression
    where I: Iterator<Item=Token> {
    match eq.next().unwrap() {
        Token::Number(c) => Expression::Number(c),
        _ => panic!("Error")
    }
}