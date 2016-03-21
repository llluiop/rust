#deref

标准库提供了一个特殊的特性， Deref 。它一般用来重载 * ，解引用运算符：

    use std::ops::Deref; //重要，解引用

    struct DerefExample<T> {
        value: T,
    }

    impl<T> Deref for DerefExample<T> {  //重载解引用
        type Target = T;

        fn deref(&self) -> &T {
            &self.value
        }
    }

    fn main() {
        let x = DerefExample { value: 'a' };
        assert_eq!('a', *x);
    }


##解引用强制多态（deref coercions）

规则如下：如果你有一个 U 类型，和它的实现 Deref<Target=T> ，（那么） &U 的值将会自动转换为 &T 。这是一个例子：


    fn foo(s: &str) {
        // borrow a string for a second
    }

    // String implements Deref<Target=str>
    let owned = "Hello".to_string();

    // therefore, this works:
    foo(&owned);
    
    U=String Deref<Target=str>, &String->&str->foo(s);
    
    
##Deref 和方法调用

当调用一个方法时 Deref 也会出现。考虑下面的例子：

    struct Foo;

    impl Foo {
        fn foo(&self) { println!("Foo"); }
    }

    let f = &&Foo;

    f.foo();
    
即便 f 是 &&Foo ，而 foo 接受 &self ，这也是可以工作的。因为这些都是一样的：

    f.foo();
    (&f).foo();
    (&&f).foo();
    (&&&&&&&&f).foo();




