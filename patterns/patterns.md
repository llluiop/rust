#patterns

##模式

###模式陷阱：
    
    let x = 1;
    let c = 'c';

    match c {
        x => println!("x: {} c: {}", x, c),  //x屏蔽了外部的x
    }

    println!("x: {}", x)
    这会打印：
    x: c c: c
    x: 1
    
###多重模式

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),  //使用‘|’进行多个匹配
        3 => println!("three"),
        _ => println!("anything"),
    }
    
###解构

    struct Point {
        x:i32,
        y:i32,
        
    }
    
    let p = Point(1,1);
    
    match p {
        Point{x,y} => println!("{},{}" x, y),
        //Point { x: x1, y: y1 } => println!("({},{})", x1, y1),  重命名
        //Point { x, .. } => println!("x is {}", x), 部分值
        
        
###忽略绑定

    # let some_value: Result<i32, &'static str> = Err("There was an error");
    match some_value {
        Ok(value) => println!("got a value: {}", value),
        Err(_) => println!("an error occurred"),  //使用‘_’忽略绑定的值
    }
    
    fn coordinate() -> (i32, i32, i32) {
    // generate and return some sort of triple tuple
    # (1, 2, 3)
    }

    let (x, _, z) = coordinate();    //忽略2
    
###ref和ref mut

    let mut x = 5;

    match x {
        ref mut mr => println!("Got a mutable reference to {}", mr),
        ref r => println!("Got a  reference to {}", r),
    }
    
    
###范围

    let x = 1;

    match x {
        1 ... 5 => println!("one through five"),//使用...匹配一个范围，用于整数，或者char:‘a’...'z'
        _ => println!("anything"),
    }
    
###绑定

    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }

    let name = "Steve".to_string();
    let mut x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
        _ => {}
    }  
    
    
    多重匹配时绑定：
    let x = 5;

    match x {
        e @ 1 ... 5 | e @ 8 ... 10 => println!("got a range element {}", e),
        _ => println!("anything"),
    }
    
    
###守卫

    //使用if
    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(5);

    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }    