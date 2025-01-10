use std::io;
fn main(){
    println!("Guess the number !");
    println!("Please input Your guess Number ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");
    println!("Your Guess : {}" , guess)
}