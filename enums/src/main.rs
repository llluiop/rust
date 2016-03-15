fn main() {    
    enum Message {
        Delivery,
        Missing,
        Move { x: i32, y: i32 },
        Go(String),
    }
    
    let x: Message = Message::Move { x: 3, y: 4 }; 
    
    match x{
        Message::Delivery => println!("Delivery"),
        Message::Missing => println!("Missing"),
        Message::Move{x, y} => println!("Move:{} {}", x, y),
        Message::Go(str) => println!("{}", str),
    }
}
