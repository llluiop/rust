use std::io;

fn main() {
    println!("Guess!");
    println!("Input your guess:");
    
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    
    println!("your guessed:{}", guess);
}
