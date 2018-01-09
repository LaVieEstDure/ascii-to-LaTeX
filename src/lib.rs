mod token;
mod lexer;
mod tests;
mod parser;

pub fn ast(text: &str) -> parser::Expression {
    let mut tokens = lexer::Lexer::new(&text);
    parser::parse(&mut tokens).unwrap()
}