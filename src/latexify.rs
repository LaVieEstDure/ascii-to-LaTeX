use parser::{Operation, Expression};
use token::Token;


pub fn latexify(expr: Expression) -> String {
    match expr {
        Expression::Number(a) => a,
        Expression::Indent(a) => a,
        Expression::BinaryExpr(a, b, c) => match_binexpr(*a, b, *c),
        Expression::Unary(Token)
        _ => panic!("no")
    }
}

// Prepare for the {{{HORRORS}}} more horrible than you've ever seen
pub fn match_binexpr(first: Expression, t: Operation, last: Expression) -> String {
    if t == Operation::Divide { 
        return format!(
                   "\\frac{{{}}}{{{}}}", latexify(first), latexify(last)
                   )
    }
    let op = match t {
        Operation::Add => "+",
        Operation::Subtract => "-",
        Operation::Multiply => "",
        _ => unreachable!()
    };
    format!("{{{}}}{}{{{}}}", latexify(first), op.to_string() , latexify(last))
}