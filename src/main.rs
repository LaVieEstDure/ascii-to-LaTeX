mod token;
mod lexer;
mod tests;

fn main(){
    let mut x = lexer::Lexer::new("1/ ");
    println!("{:?}, {:?}, {:?}", x.next_token(),x.next_token(),x.next_token());
}