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
    Divide,
    Function,
    Add,
    Subtract,
    Multiply
}

struct Parser<'a> {
    equation: lexer::Lexer<'a>
}

fn get_priority(token: &Token) -> u8 {
    match *token {
        Token::Multiply | Token::Divide => 3,
        Token::Plus | Token::Minus => 2,
        _ => 1
    }
}

fn is_infix_op(token: &Token) -> bool {
    match *token {
        Token::Plus
        | Token::Minus
        | Token::Multiply
        | Token::Divide => true,
        
        _ => false
    }
}

pub fn parse (eq: &mut lexer::Lexer) -> Result<Expression, String> {
    parse_expression(&mut eq.peekable(), 0)
}

fn parse_expression<I> (eq: &mut Peekable<I>, priority: u8) -> Result<Expression, String>
    where I: Iterator<Item=Token> {
    let mut expression = parse_prefix_expr(eq).unwrap();
    let mut next_priority: u8;
    loop {
        if let Some(next_token) = eq.peek() {
            next_priority = get_priority(&next_token);

            if priority >= next_priority {
                break
            }
        } else{break;}
        expression = parse_infix_expr(expression, eq, next_priority).unwrap()
    }
    Ok(expression)
}

fn parse_prefix_expr<I>(eq: &mut Peekable<I>) -> Result<Expression, String>
    where I: Iterator<Item=Token> {
    match eq.next() {
        Some(Token::Number(c)) => Ok(Expression::Number(c)),
        Some(f @ Token::Sin)
        | Some(f @ Token::Cos)
        | Some(f @ Token::Tan) => Ok(parse_function(f, eq).unwrap()),
        _ => Err("Invalid token".to_string())
    }
}

fn parse_function<I>(function: Token, eq: &mut Peekable<I>) -> Result<Expression, String>
    where I: Iterator<Item=Token> {
    

    Err("lol".to_string())
}

fn parse_infix_expr<I>(first: Expression, eq: &mut Peekable<I>, priority: u8) -> Result<Expression, String>
    where I: Iterator<Item=Token> {
    
    match eq.next() {
        Some(token) => {
            if is_infix_op(&token){
                let op = match token {
                    Token::Plus => Operation::Add,
                    Token::Minus => Operation::Subtract,
                    Token::Divide => Operation::Divide,
                    Token::Multiply => Operation::Multiply,
                    _ => panic!("Not operator")
                };
                let right = parse_expression(eq, priority).unwrap();
                Ok(Expression::BinaryExpr(Box::new(first), op, Box::new(right)))
            } else {
                Err(format!("Not operator: {:?}", token).to_string())
            }
        },
        _ => Err("No more tokens".to_string())
    }
}