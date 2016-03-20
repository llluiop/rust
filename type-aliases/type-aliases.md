#type-aliases

type 关键字让你定义另一个类型的别名：

    type Name = String;
    let x: Name = "Hello".to_string();
    
    
你也可以在泛型中使用类型别名：

    use std::result;

    enum ConcreteError {
        Foo,
        Bar,
    }

    type Result<T> = result::Result<T, ConcreteError>;
