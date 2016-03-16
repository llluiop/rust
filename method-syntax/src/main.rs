struct Foo {
    x :i32,
    y :i32,
}

impl Foo {
    fn new(x:i32, y:i32) -> Foo {
        Foo{x:x, y:y,}
    }
    
    fn setx(&mut self, x:i32){
        self.x = x;
    }
}


fn main() {
    let f = &mut Foo::new(1,2);
    println!("{}, {}", f.x, f.y);
    
    f.setx(2);
    println!("{}, {}", f.x, f.y);    
}
