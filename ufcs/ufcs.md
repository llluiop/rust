#ufcs

##函数同名

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

    let b = Baz;
    
    b.f() //error: multiple applicable methods in scope [E0034]
    
    
    这里我们使用通用函数调用语法utfs;
    
    Foo::f(&b);  //self作为第一个参数，和python很像，又是c++类成员函数隐式的实现方式
    Bar::f(&b);
    
##尖括号形式

    一般的ufsc: Trait::method(args)
    
    扩展形式：<Type as Trait>::method(args);
    
    
    trait Foo {
    fn foo() -> i32;
    }

    struct Bar;

    impl Bar {
        fn foo() -> i32 {
            20
        }
    }

    impl Foo for Bar {
        fn foo() -> i32 {
            10
        }
    }

    fn main() {
        assert_eq!(10, <Bar as Foo>::foo());
        assert_eq!(20, Bar::foo());
    }