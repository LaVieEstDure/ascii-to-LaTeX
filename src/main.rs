#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]


mod token;
mod lexer;
mod tests;
mod parser;


fn main(){
    let text = String::from("3.14 sin(3) +2");
    let x = lexer::Lexer::new(&text);
    let p = parser::parse(x.peekable());
    println!("{:?}", p);
}