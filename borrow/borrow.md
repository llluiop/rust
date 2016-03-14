#borrow and reference

##借用

承接上文：

    fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
        // do stuff with v1 and v2

        // hand back ownership, and the result of our function
        (v1, v2, 42)
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let (v1, v2, answer) = foo(v1, v2);
    
对于这样的情况，我们现在有一个处理方式，那就是借用：

    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        // do stuff with v1 and v2

        // return the answer
        42
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let answer = foo(&v1, &v2); //借用

    // we can use v1 and v2 here!
    
因为借用的缘故，函数内的v1和v2在超出函数作用域的时候不会被释放，但是需要思考一个问题，此时如果在函数内
**改变了引用**，那么外界是不知道的，这会有隐式的数据竞争在，所以我们规定：**引用是不可变的**：

    fn foo(v: &Vec<i32>) {
        v.push(5);
    }

    let v = vec![];
    foo(&v);

    error: cannot borrow immutable borrowed content `*v` as mutable
    v.push(5);
    ^
    
##&mut引用

&mut允许你改变引用的对象，前提是引用的对象自身也必须是**可变的**：

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }    
    
    println!("{}", x);
    
    
* &T这种类型的引用可以有0或N个    
* &mut T这种类型的引用只能存在一个

&T因为借用发生后均不可变，所以可以存在多个
&mut T因为借用后可以变，如果一个发生了变化，另一个无法更新，此时会出现数据竞争，所以只能有一个

##一些常犯错误

* 迭代器失效

        let mut v=vec![1, 2, 3];
        for i in &v {  //这里做了borrow
            println!("{}", i);
            v.push(4);  //错误！
        }
        防止迭代过程中，迭代器失效
    
* 释放后使用

        let y: &i32;
        let x = 5;
        y = &x;

        println!("{}", y);

