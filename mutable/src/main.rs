use std::sync::Arc;
use std::cell::RefCell;

fn main() {
    let mut x = 1;
    foo(x);
}

fn foo(mut x : i32)
{
    x= 1;
    println!("{}", x);
}

fn Interior()
{
    let x = Arc::new(5);
    let y = x.clone();      
}

fn Exterior()
{
    let x = RefCell::new(42);

    let y = x.borrow_mut();
    let z = x.borrow_mut();
}