#generics

##泛型

强类型语言里的泛型似乎都有，只要是为了节省代码量，统一编程模型，rust也是如此：

    enum Result<A, Z> {
        Ok(A),
        Err(Z),
    }
    
##泛型函数

    fn takes_two_things<T, U>(x: T, y: U) {
        // ...
    }
    
##泛型结构体

    struct Point<T> {
        x: T,
        y: T,
    }

    let int_origin = Point { x: 0, y: 0 };
    let float_origin = Point { x: 0.0, y: 0.0 };    