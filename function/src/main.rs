fn main() {
    print_number(5);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn foo(x: i32) -> i32 {
    return x + 1;
}

fn foo2(x: i32) {
    x + 1;
}
