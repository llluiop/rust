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


##多 trait bound（Multiple trait bounds）

    fn foo<T: Traits1 + Traits2>(x: T) {   //T =  Traits1 + Traits2
    }


    
