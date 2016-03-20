trait Foo { fn method(&self); }
impl Foo for u8 {
     fn method(&self) { println!("u8"); }
}

impl Foo for String {
     fn method(&self) {  println!("string"); }
}

fn do_something<T: Foo>(x: T) {
    x.method();
}

fn do_something_dynamic(x:&Foo)
{
    x.method();
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    do_something(x);
    do_something(y);
    
    let x = 6u8;
    do_something_dynamic(&x as &Foo);
    do_something_dynamic(&x);
}