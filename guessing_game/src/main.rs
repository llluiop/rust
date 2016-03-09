use std::io;
use std::cmp::Ordering;
use rand::Rng;

extern crate rand;

fn main() {
    println!("Guess!");
    println!("Input your guess:");
    
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    
    println!("your guessed:{}", guess);
}
