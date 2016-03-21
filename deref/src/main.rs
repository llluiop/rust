fn main() {
    let f = &&Foo;    
    let ff = &f;

    ff.foo();    
    
    println!("Hello, world!");
}


    struct Foo;

    impl Foo {
        fn foo(&self) { println!("Foo"); }
    }
