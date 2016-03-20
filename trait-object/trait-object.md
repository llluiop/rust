#trait-object

##静态分发

    # trait Foo { fn method(&self) -> String; }
    # impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
    # impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }
    fn do_something<T: Foo>(x: T) {
        x.method();
    }

    fn main() {
        let x = 5u8;
        let y = "Hello".to_string();

        do_something(x);
        do_something(y);
    }
    
    对于do_something()，rust会使用静态分发的方式，这和我们在c++里认知的概念是类似的，模板会由编译器展开
    也就是将会为x和y分别产生不同的代码：
    
    fn do_something_u8(x: u8) {
    x.method();
    }

    fn do_something_string(x: String) {
        x.method();
    }
    
    
##动态分发    

> Rust 通过一个叫做“trait 对象”的功能提供动态分发。比如说 &Foo 、 Box<Foo> 这些就是trait对象。
它们是一些值，值中储存实现了特定 trait 的任意类型。它的具体类型只能在运行时才能确定，比如&Foo，类似于
c++中的基类，u8和string是其子类。

    # trait Foo { fn method(&self) -> String; }
    # impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
    # impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }

    fn do_something(x: &Foo) {
        x.method();
    }

    fn main() {
        let x = 5u8;
        do_something(&x as &Foo);
        //do_something(&x);
    }
    
    可以看到，traits对象就是类似c++中的基类指针，并且指针的大小与其存储的对象大小是无关的。
    
什么是一个trait对象：

    # mod foo {
    pub struct TraitObject {
        pub data: *mut (),
        pub vtable: *mut (),
    }
    # }
    
    一个trait对象就像包含一个“数据”指针和“虚函数表”指针的 &Foo，这c++又不谋而合
    
    
##对象安全
    
并不是所有 trait 都可以被用来作为一个 trait 对象。
例如，vector 实现了 Clone ，不过如果我们尝试创建一个 trait 对象：
    
    let v = vec![1, 2, 3];
    let o = &v as &Clone;
    
我们得到一个错误：

    error: cannot convert to a trait object because trait `core::clone::Clone` is not object-safe [E0038]
    let o = &v as &Clone;
            ^~
    note: the trait cannot require that `Self : Sized`
    let o = &v as &Clone;

一个对象安全的 trait 需要如下两条为真：

* trait 并不要求 Self: Sized 
* 所有的方法是对象安全的

一个直观的理解是“除了特殊情况，如果你的 trait 的方法使用了 Self ，它就不是对象安全的。
