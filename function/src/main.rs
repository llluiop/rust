fn main() {
    foo(5);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn foo(x: i32) -> i32 {
    x + 1
    
    //println!("x is: {}", x);  //error
}

fn foo2(x: i32) {
    x + 1;
}
