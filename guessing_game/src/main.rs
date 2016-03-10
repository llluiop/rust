use std::io;
use std::cmp::Ordering;
use rand::Rng;

extern crate rand;

fn main() {
    println!("Guess!");

    
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret_number is: {}", secret_number);
    
    let mut guess = String::new();
    
    println!("Input your guess:");
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    
    println!("your guessed:{}", guess);
}
