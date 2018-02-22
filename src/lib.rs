mod token;
mod lexer;
mod tests;
mod parser;
mod latexify;
use parser::Expression;

pub fn ast(text: &str) -> parser::Expression {
    let mut tokens = lexer::Lexer::new(&text);
    parser::parse(&mut tokens).unwrap()
}

pub fn latex(text: &str) -> String {
    latexify::latexify(ast(text))
}