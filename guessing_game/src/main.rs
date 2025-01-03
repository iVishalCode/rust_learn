use std::io;
fn main(){
    println!("Guess the Number");
    println!("Please enter the guess number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to learn the read line");
    println!(" You guess : {}" , guess)
}