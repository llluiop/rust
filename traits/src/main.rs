fn main() {    
    print(Circle{radius:1});
    
        let mut r = Rectangle {
        x: 0,
        y: 0,
        width: 47,
        height: 47,
    };

    assert!(r.is_square());
}

trait HasArea {
    fn area(&self)->i32;
}

struct Circle {
    radius:i32,
}



impl HasArea for Circle {
    fn area(&self)->i32{
        self.radius * self.radius
    }
}

fn print<T:HasArea>(shape:T){
    println!("{}", shape.area());
}


struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

impl<T> Rectangle<T> {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}




