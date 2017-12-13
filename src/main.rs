mod token;
mod lexer;
mod tests;
// mod parser;
use std::io;
extern crate time;
use time::PreciseTime;



fn main(){
    let mut text = String::from("3.14 + sin(pi/2)");
    println!("\nTOKENS FOUND:");
    let start = PreciseTime::now();
    for i in lexer::Lexer::new(&text) {
        println!("   {:?}", i);
    }
    let end = PreciseTime::now();
    println!("{} for parsing", start.to(end));
}