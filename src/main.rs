extern crate ascii2latex;
use ascii2latex::*;

fn main(){
    let input = "3*(4+2)";
    println!("\n\nINPUT WAS:\n    {}\n", input);
    println!("ANSWER WAS:\n{:#?}\n", ast("3*(4+2)"));
}
