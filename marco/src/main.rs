fn main() {
    let x:i32 = {let x:i32 = 5; x};
    
    println!("{}", x);
}
