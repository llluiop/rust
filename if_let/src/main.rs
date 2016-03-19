fn main() {
    let x = Some(5);
    fn foo(x:i32){println!("{}", x);}
    
    match x{
        Some(x) => {foo(x)},
        None=>{}
    }
    
    if x.is_some() {
        let x = x.unwrap();
        foo(x);
    }   
    
    if let Some(x) = x{
        foo(x);
    }
    else
    {
        println!("don't match");
    }
}
