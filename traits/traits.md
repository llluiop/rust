#tratis

**trait 是一个告诉 Rust 编译器一个类型必须提供哪些功能语言特性。**
其实就是模板约束，c++的[concept](http://blog.csdn.net/pongba/article/details/1726031)


##泛型函数的 trait bound（Trait bounds on generic functions）

    fn print_area<T>(shape: T) {
        println!("This shape has an area of {}", shape.area());
    }


    Rust抱怨道：
    error: no method named `area` found for type `T` in the current scope
    
还记不记得generics那一章，print x编译报错的问题，就是这个原因，解决办法：
    
    trait HasArea {
        fn area(&self) -> f64;
    }

    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }


    fn print_area<T: HasArea>(shape: T) {
        println!("This shape has an area of {}", shape.area());
    }
    
    
##泛型结构体的 trait bound（Trait bounds on generic structs）    

    struct Rectangle<T> {
        x: T,
        y: T,
        width: T,
        height: T,
    }

    impl<T: PartialEq> Rectangle<T> {
        fn is_square(&self) -> bool {
            self.width == self.height  //'=='的实现基于 core::cmp::PartialEq  trait 的类型，这是一个标准库实现类型，如果没有的话，会报错
        }
    }

    fn main() {
        let mut r = Rectangle {
            x: 0,
            y: 0,
            width: 47,
            height: 47,
        };

        assert!(r.is_square());
    }


##实现 trait 的规则（Rules for implementing traits）

    trait HasArea {
        fn area(&self) -> f64;
    }

    impl HasArea for i32 {
        fn area(&self) -> f64 {
            println!("this is silly");  //改写标准类型，不推荐！
            *self as f64
        }
    }
    
    在基本类型上实现方法被认为是不好的设计，即便这是可以的。

标准库提供了一个 Write trait来为 File 增加额外的功能。默认， File 并不会有这个方法：

    let mut f = std::fs::File::open("foo.txt").ok().expect("Couldn’t open foo.txt");
    let buf = b"whatever"; // byte string literal. buf: &[u8; 8]
    let result = f.write(buf);
    # result.unwrap(); // ignore the error


    这里是错误：

    error: type `std::fs::File` does not implement any method in scope named `write`
    let result = f.write(buf);
               ^~~~~~~~~~


我们需要先 use 这个 Write  trait：

    use std::io::Write;  //增加额外的traits

    let mut f = std::fs::File::open("foo.txt").expect("Couldn’t open foo.txt");
    let buf = b"whatever";
    let result = f.write(buf);
    # result.unwrap(); // ignore the error


    这样就能无错误的编译了。

这意味着即使有人做了像给 int 增加函数这样的坏事，它也不会影响你，除非你 use 了那个trait。

这还有一个实现trait的限制。不管是trait还是你写的 impl 都只能在你自己的包装箱内生效。
所以，我们可以为 i32 实现 HasArea trait，因为 HasArea 在我们的包装箱中。
不过如果我们想为 i32 实现 Float trait，它是由Rust提供的，则无法做到，因为这个trait和类型都不在我们的包装箱中。

> 带有trait限制的泛型函数是单态（monomorphization）（mono：单一，morph：形式）的，所以它是静态分发（statically dispatched）的


##多trait bound（Multiple trait bounds）

    fn foo<T: Traits1 + Traits2>(x: T) {   //T =  Traits1 + Traits2
    }
    
    
##where从句    

    fn foo<T: Traits1 + Traits2>(x: T) {   
    }
    
    ==>
    
    fn foo<T>(x: T) where T:Traits1 + Traits2  {  
    }
    
    
    更好的traits：
    
    trait ConvertTo<Output> {   //traits自身也可以是一个模板
        fn convert(&self) -> Output;
    }

    impl ConvertTo<i64> for i32 {
        fn convert(&self) -> i64 { *self as i64 }
    }

    // can be called with T == i32
    fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
        x.convert()
    }

    // can be called with T == i64
    fn inverse<T>() -> T
        // this is using ConvertTo as if it were "ConvertTo<i64>"
        where i32: ConvertTo<T> {
        42.convert()
    }
    
    
##默认方法

    # trait Foo {
    #     fn is_valid(&self) -> bool;   //必须实现
    #
    #     fn is_invalid(&self) -> bool { !self.is_valid() }  //默认实现
    # }
    struct UseDefault;

    impl Foo for UseDefault {
        fn is_valid(&self) -> bool {
            println!("Called UseDefault.is_valid.");
            true
        }
    }

    struct OverrideDefault;

    impl Foo for OverrideDefault {
        fn is_valid(&self) -> bool {
            println!("Called OverrideDefault.is_valid.");
            true
        }

        fn is_invalid(&self) -> bool {   //覆盖默认实现
            println!("Called OverrideDefault.is_invalid!");
            true // overrides the expected value of is_invalid()
        }
    }

    let default = UseDefault;
    assert!(!default.is_invalid()); // prints "Called UseDefault.is_valid."

    let over = OverrideDefault;
    assert!(over.is_invalid()); // prints "Called OverrideDefault.is_invalid!"  
    
##继承

 FooBar 的实现也必须实现 Foo ，像这样：

    # trait Foo {
    #     fn foo(&self);
    # }
    # trait FooBar : Foo {   //FooBar继承了Foo
    #     fn foobar(&self);
    # }
    struct Baz;

    impl Foo for Baz {      
        fn foo(&self) { println!("foo"); }
    }

    impl FooBar for Baz {    //因为使用了FooBar，所以上面必须实现Foo
        fn foobar(&self) { println!("foobar"); }
    }


    如果我们忘了实现 Foo ，Rust会告诉我们：
    error: the trait `main::Foo` is not implemented for the type `main::Baz` [E0277]
    
##派生

重复的实现像 Debug 和 Default 这样的 trait 会变得很无趣。为此，Rust 提供了一个属性来允许我们让 Rust 为我们自动实现 trait：

    #[derive(Debug)]
    struct Foo;

    fn main() {
        println!("{:?}", Foo);
    }
    
      
    


    
