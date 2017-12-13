#[derive(Debug,PartialEq)] 
pub enum Expression {
    Unary(Operation, Box<Expression>)
    BinaryExpr(Box<Expression>, Operation, Box<Expression>)
}

pub enum Operation {
    Fraction,
    Function,
}

struct Parser {

}