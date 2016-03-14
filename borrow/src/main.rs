fn main() {
    let mut x = 5;
    {
        let y = &mut x;
        //x= 6; //error
        *y += 1;  //y是不可变的，只能是x的可变引用，但是对y取值是可以变的，因为取值后是x，x是可变的
    }    
    
    println!("{}", x);
}

fn borrow_can_not_change()
{
    let x = 5;
    let y = &x;    
    //x = 6;  //error
    //*y = 6;  //error
    
    /**************/
    let mut a = 5;
    let b = &a;
    
    //a = 7;  //error
}

fn iterator()
{
    let mut v=vec![1, 2, 3];

    for i in &v {  //这里做了borrow
        println!("{}", i);
        //v.push(4);  //错误！
    }
    
    /*
    let mut v=vec![1, 2, 3];

    for i in v {  //如果这里使用v，而不是引用，那么v这里发生move语义，同样报错
        println!("{}", i);
        v.push(4);  //错误！
    }
    */
}

fn live_long_enough()
{
    let y: &i32;
    let x = 5;
    //y = &x; //error
}
