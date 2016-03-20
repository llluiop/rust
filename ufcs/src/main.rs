    trait Foo {
        fn f(&self);
    }

    trait Bar {
        fn f(&self);
    }

    struct Baz;

    impl Foo for Baz {
        fn f(&self) { println!("Baz’s impl of Foo"); }
    }

    impl Bar for Baz {
        fn f(&self) { println!("Baz’s impl of Bar"); }
    }


fn main() {
    let b = Baz;
    
    <Baz as Foo>::f(&b);    
    
}
