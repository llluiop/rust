#const and static

##const常量

    const N:i32 = 5;  //内联，无固定内存地址,贯穿程序整个生命周期
    
##static

    static N: i32 = 5; //不内联，有固定的内存地址，贯穿整个生命周期
    
    任何存储在常量中的引用有一个 'static 生命周期：
    static NAME: &'static str = "Steve";
    
    
##可变性

你可以用 mut 关键字引入可变性：

    static mut N: i32 = 5;


因为这是可变的，一个线程可能在更新 N 同时另一个在读取它，导致内存不安全。因此访问和改变一个 static mut 是不安全（unsafe）的，因此必须在 unsafe 块中操作：
    # static mut N: i32 = 5;

    unsafe {
        N += 1;

        println!("N: {}", N);
    }

更进一步，任何存储在 static 的类型必须实现 Sync 。
    
##初始化

const 和 static 都要求赋予它们一个值。它们只能被赋予一个常量表达式的值。
换句话说，你不能用一个函数调用的返回值或任何相似的复合值或在运行时赋值。

如果你可以在两者之间选择，选择 const 



    