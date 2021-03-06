use token::Token;
use std::iter::Peekable;
use lexer;

// TODO: Rename Operation to InfixOp and have a separate enum
// for other kinds of operations (functions et al)
// The fact that Expressions are split between multiple 
// Enums is really confusing and hard to follow

#[derive(Debug,PartialEq)] 
pub enum Expression {
    Indent(String),
    Number(String),
    Constant(Token),
    Unary(Token, Box<Expression>),
    BinaryExpr(Box<Expression>, Operation, Box<Expression>),
    Boxed(Box<Expression>)
}

#[derive(Debug,PartialEq)] 
pub enum Operation {
    Divide,
    Add,
    Subtract,
    Multiply,
    Pow,
    Sub,
    Equals,
    Less,
    LessEq,
    Greater,
    GreaterEq
}

pub fn parse (eq: &mut lexer::Lexer) -> Result<Expression, String> {
    parse_expression(&mut eq.peekable(), 0, false)
}

fn skip_whitespace<I> (eq: &mut Peekable<I>) 
    where I: Iterator<Item=Token> {
    loop {
        if let Some(&Token::Whitespace) = eq.peek() {
            eq.next();
        }
        else { break; } 
    }
}

fn get_priority(token: &Token) -> u8 {
    match *token {
        Token::Super => 4,
        Token::Multiply 
        | Token::Divide
        | Token::Frac => 3,
        Token::Plus | Token::Minus => 2,
        _ => 1
    }
}

fn parse_expression<I> (eq: &mut Peekable<I>, priority: u8, parenth_opened: bool) 
    -> Result<Expression, String> where I: Iterator<Item=Token> {
    
    let mut expression = parse_prefix_expr(eq).unwrap();
    let mut next_priority: u8;
    loop {
        if let Some(next_token) = eq.peek() {
            next_priority = get_priority(&next_token);
            if parenth_opened && next_token == &Token::RightParenth {
                break;
            }
            if priority >= next_priority {
                break;
            }
        } else{ break; }
        
        expression = parse_infix_expr(expression, eq, next_priority).unwrap()
    }
    if parenth_opened { eq.next(); }
    Ok(expression)
}

fn parse_prefix_expr<I>(eq: &mut Peekable<I>) -> Result<Expression, String>
    where I: Iterator<Item=Token> {
    match eq.next() {
        Some(Token::Number(c)) => Ok(Expression::Number(c)),
        Some(Token::Indent(c)) => Ok(Expression::Indent(c)),
        Some(Token::LeftParenth) => Ok(Expression::Boxed(Box::new(parse_expression(eq, 0, true).unwrap()))),
        Some(f @ Token::Sin)
        | Some(f @ Token::Cos)
        | Some(f @ Token::Tan)
        | Some(f @ Token::Sqrt) => Ok(parse_function(f, eq).unwrap()),
        f @ _ => Err(String::from(format!("Invalid token: {:?}", f)))
    }
}

fn is_infix_op(token: &Token) -> bool {
    match *token {
        Token::Plus
        | Token::Minus
        | Token::Multiply
        | Token::Divide
        | Token::Frac
        | Token::Super
        | Token::Sub
        | Token::Equal
        | Token::Greater
        | Token::GreatEq
        | Token::Less
        | Token::LessEq => true,
        
        _ => false
    }
}

fn parse_infix_expr<I>(first: Expression, eq: &mut Peekable<I>, priority: u8) -> Result<Expression, String>
    where I: Iterator<Item=Token> {
    
    match eq.next() {
        Some(token) => {
            if is_infix_op(&token){
                let op = match token {
                    Token::Plus => Operation::Add,
                    Token::Minus => Operation::Subtract,
                    Token::Divide | Token::Frac => Operation::Divide,
                    Token::Multiply => Operation::Multiply,
                    Token::Super => Operation::Pow,
                    Token::Sub => Operation::Sub,
                    Token::Equal => Operation::Equals,
                    Token::Less => Operation::Less,
                    Token::LessEq => Operation::LessEq,
                    Token::Greater => Operation::Greater,
                    Token::GreatEq => Operation::GreaterEq,
                    _ => panic!("Not operator")
                };
                let right = parse_expression(eq, priority, false).unwrap();
                Ok(Expression::BinaryExpr(Box::new(first), op, Box::new(right)))
            } else {
                Err(format!("Not operator: {:?}", token).to_string())
            }
        },
        _ => Err(String::from("No more tokens"))
    }
}

fn parse_function<I>(function: Token, eq: &mut Peekable<I>) -> Result<Expression, String>
    where I: Iterator<Item=Token> {

    let expr: Expression;
    skip_whitespace(eq);
    if let Some(Token::LeftParenth) = eq.next(){
        expr = parse_expression(eq, 0, true).unwrap();
    } else {panic!("Function not called: Recieved token {:?}", eq.next())}

    Ok(Expression::Unary(function,Box::new(expr)))
}
