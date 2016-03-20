#attributes

在Rust中声明可以用“属性”标注，它们看起来像：
    #[test]
    # fn foo() {}
    
    //or
    # mod foo {
    #![test]
    # }
    
    
你不能创建你自己的属性，Rust编译器定义了它们。
