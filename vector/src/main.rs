fn main() {    
    let v = vec![1, 2, 3];
    match v.get(1) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }   
    
    
    println!("Hello, world!");
}
