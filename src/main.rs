mod token;
mod lexer;
mod tests;
mod parser;



fn main(){
    let text = String::from("3.14+2");
    let x = lexer::Lexer::new(&text);
    let p = parser::parse(x.peekable());
    println!("{:?}", p);
}