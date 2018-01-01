#![allow(unused_variables)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]


mod token;
mod lexer;
mod tests;
mod parser;

fn main(){
    let text = String::from("3+2*2");
    let mut x = lexer::Lexer::new(&text);
    let p: parser::Expression = parser::parse(&mut x).unwrap();
}