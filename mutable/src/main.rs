fn main() {
    let mut x = 1;
    foo(x);
}

fn foo(mut x : i32)
{
    x= 1;
    println!("{}", x);
}