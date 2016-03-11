#function

##参数类型与返回值

    fn add_one(x: i32) -> i32 {
        x + 1  //没有分号！
    }
    
> 关于 Rust 两个有趣的地方：它是一个基于表达式的语言，并且分号与其它基于“大括号和分号”的语言不同?

##表达式和语句

* 表达式返回值
* 语句不返回值

> x+1 是表达式，所以返回值， x+1;是语句，不返回值

> let x = 1; 是一个语句

> let mut x =1; let y =(x = 2); 后一句x=2将会是一个表达式，返回值是一个空的元组'()'

> 使用return提早返回

##发散函数

Rust有些特殊的语法叫“发散函数”，这些函数并不返回：

    fn diverges() -> ! {
        panic!("This function never returns!");
    }
    
    因为这个函数会崩溃，所以它不会返回，所以它拥有一个类型!
    
    发散函数可以被用作任何类型：
    let x: i32 = diverges();
    let x: String = diverges();
    
    
##函数指针

    # fn plus_one(i: i32) -> i32 { i + 1 }
    # let f = plus_one;
    let six = f(5);
