#match

##取代if/else

    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),  
    }
    
    穷尽性检查（exhaustiveness checking）：
        如果没有最后的‘_’, 编译器error: non-exhaustive patterns: `_` not covered
    
    支持赋值：
        let number = match x {...}