extern crate ascii2latex;
use ascii2latex::*;

fn main(){
    // let inputs = vec!("1^2^3", "2*sin(3)", "3=4", "sqrt(2)/2");
    // for i in inputs.iter() {
    //     println!("\nInput was: {}\nOutput was: {:#?}", i, ast(i));
    // }
    
    println!("{}", latex("2/2*2"));
}
