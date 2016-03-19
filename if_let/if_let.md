#if let

> if let 允许你合并 if 和 let 来减少特定类型模式匹配的开销。

    let x = Some(5);
    fn foo(x:i32){println!("{}", x);}
    
    match x{
        Some(x) => {foo(x)},
        None=>{}
    }
    
    //轻量的match
    if x.is_some() {
        let x = x.unwrap();
        foo(x);
    }   
    
    //更简洁的写法
    if let Some(x) = x{
        foo(x);
    }
    else
    {
        println!("don't match");
    }
    
    
#while let

>   当你想一直循环，直到一个值匹配到特定的模式的时候，你可以选择使用 while let 。使用 while let 可以把类似这样的代码：

    let mut v = vec![1, 3, 5, 7, 11];
    loop {
        match v.pop() {
            Some(x) =>  println!("{}", x),
            None => break,
        }
    }


    变成这样的代码：
    let mut v = vec![1, 3, 5, 7, 11];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
    