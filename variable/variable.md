#variable
##显式可变
> let x = 5;

> x = 6; //compiler error

> let mut x = 5; x = 6; //ok

##静态类型

> rust是静态类型语言，let可以使用的原因是rust会进行类型推断，是不是和c++11的auto很像！
> 可以指定类型： let x:i32 = 5;

##初始化

> 声明时不初始化，如果下面不使用，会得到一个编译警告，如果使用那么会编译失败，不同于c/c++

##作用域和隐藏

> fn main() {
>    let x: i32 = 17;
>    {
>        let y: i32 = 3;
>        println!("The value of x is {} and value of y is {}", x, y);
>    }
>    println!("The value of x is {} and value of y is {}", x, y); // This won't work
>}