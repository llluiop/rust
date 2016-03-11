#loop

一共三种迭代：
* loop
* while
* for

##loop

    loop {
        println!("Loop forever!");
    }


##while

    while true{
    }
        
    let mut done = false;
    while !done{
        done = true;
    }
        
##for

    for x in 0..10 {
        println!("{}", x); // x: i32
    }
    
    0..10是一个迭代器表达式：
    for var in expression {
        code
    }
    
    
##循环计数

    for (count,element) in (15..20).enumerate() {
        println!("count = {} and element = {}", count, element);
    }
    
    count藉由enumerate实现自增长计数
    
##提早结束迭代（Ending iteration early）

    使用break和continue，不赘述
    
    跳出至标签：
    'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // continues the loop over x
        if y % 2 == 0 { continue 'inner; } // continues the loop over y
        println!("x: {}, y: {}", x, y);
    }
    }
    
    按照之前结构化编程的思想，这不是一个好的习惯。