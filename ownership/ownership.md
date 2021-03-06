#ownership 所有权

##rust为了为了实现内存安全，实现了三个系统，系统包含的一些概念：
* 所有权
* 借用
* 生命周期


##原则
*很多 Rust 初学者会经历我们所谓的“与借用检查器作斗争”的过程，也就是指 Rust 编译器拒绝编译一个作者认为合理的程序。*

的确，我已经被搞的焦头烂额了，这也是我觉得rust很难流行的一个重要原因，学习曲线陡峭。


##所有权
Rust 中的变量绑定有一个属性：它们有它们所绑定的的值的所有权。

    fn foo() {
    let v = vec![1, 2, 3];
    }
    
 当v离开作用域时，绑定的vec将会被释放，此时vec是分配在堆上的。
 
 ##移动语义
 
 Rust 确保了对于任何给定的资源都正好（只）有一个绑定与之对应：
 
 example1:
 
    let v = vec![1, 2, 3];
    let v2 = v;
    println!("v[0] is: {}", v[0]); //error
    
    note：如果v是绑定的数值或者布尔类型，那么上述代码是可以的

 example2:    
 
    fn take(v: Vec<i32>) {
    // what happens here isn’t important.
    }

    let v = vec![1, 2, 3];
    take(v);
    println!("v[0] is: {}", v[0]); //error
    
C++ 11使用std::move()实现了类似的语义，目的是实现高效，减少拷贝的次数，而rust默认就是如此，想要拷贝需要
使用clone()函数。**没有银弹**

##细节

在移动了绑定后我们不能使用它的原因是微妙的，也是重要的。当我们写了这样的代码：

    let v = vec![1, 2, 3];
    let v2 = v;
    
vec分配在堆上，v保存了一个指向他的指针，赋值后v2也保存了一个指向他的指针，此时按照rust的设计，为了禁止
出现数据竞争，前者v的引用必须无效，这也是为什么有移动语义。

##copy类型

    let v = 1;
    let v2 = v;

    println!("v is: {}", v);
    
v是一个i32，它实现了Copy，而不是移动，所有的基础类型都默认实现了copy trait，*可以为自定义类型编写traits*。


##函数的所有权

    fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
        // do stuff with v1 and v2

        // hand back ownership, and the result of our function
        (v1, v2, 42)  //函数返回值！！！
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let (v1, v2, answer) = foo(v1, v2);     //重新得到所有权
    
**Rust 提供了一个 trait，借用，它帮助我们解决这个问题**。