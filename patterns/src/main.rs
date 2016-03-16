fn main() {
    
    struct Point {
        x:i32,
        y:i32,
        
    }
    
    let p = Point{x:1, y:2};
    
    match p {
        Point{x:x1, y:y1} => println!("{}{}", x1, y1),  //可以对x重命名为x1
    }
    
    match p {
        Point{y,..} => println!("{}", y),  //部分解构
    }
    println!("Hello, world!");
    
   let mut x = 5;

    match x {
        //ref mut mr => println!("Got a mutable reference to {}", mr),
        //ref r => println!("Got a  reference to {}", r),
    }
}
