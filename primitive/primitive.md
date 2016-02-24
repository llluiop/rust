#原生类型

##bool类型
* true
* false

##char类型
> 四个字节

##数字类型
* i8
* i16
* i32
* i64
* u8
* u16
* u32
* u64
* isize //依赖底层机器指针大小
* usize //依赖底层机器指针大小
* f32
* f64

##数组

    let a = [1, 2, 3]; // a: [i32; 3]
    let mut m = [1, 2, 3]; // m: [i32; 3]
    数组的类型是[T; N], T是数组元素类型，N是个数
    
    
##切片语法

    let a = [0, 1, 2, 3, 4];
    let complete = &a[..]; // A slice containing all of the elements in a
    let middle = &a[1..4]; // A slice of a: just the elements 1, 2, and 3
    使用&与[]的组合来创建一个切片
    
##元组

    let x = (1, "hello");
    let x(i32, &str) = (1, "hello");  &str是一个字符串切片
    
> 元组包含的*元素类型*可以不一致，这一点不同于*数组*

    let (x,y,z) = (1,2,3); //解构,长度必须一致，否则编译失败
    
    let tuple = (1, 2, 3);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;
    
##函数

    函数也有一个类型！它们看起来像这样：
    fn foo(x: i32) -> i32 { x }
    let x: fn(i32) -> i32 = foo;


    
    