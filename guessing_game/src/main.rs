use rand :: Rng;

use std::cmp::Ordering;
use std::io;

fn main(){
    println!("Guess the Number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!(" Please Input Your Guess Number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read line");
    let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    println!(" Guess Number : {guess}");
    match guess.cmp(&secret_number) {
        Ordering::Greater=>println!("Too Big!"),
        Ordering :: Less =>println!(" Too small!"),
        Ordering::Equal =>{
            println!(" You Win !");
            break;
        }
    }
    }
}