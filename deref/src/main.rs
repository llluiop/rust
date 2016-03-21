fn main() {
    

    let f = &&Foo;

    f.foo();
    
    
    println!("Hello, world!");
}


    struct Foo;

    impl Foo {
        fn foo(&self) { println!("Foo"); }
    }
