#vector

##vector

vector是动态增长的数组，类型为Vec<T>， 内存分配在堆上！

重复值：
    
    let v = vec![0; 10]; //ten zeroes
    
##索引

> 索引的类型是usize

        let v = vec![1, 2, 3, 4, 5];

        let i: usize = 0;
        let j: i32 = 0;

        // works
        v[i];

        // doesn’t
        v[j];
        
        
##越界

使用get或者get_mut来处理越界：

    let v = vec![1, 2, 3];
    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }       
    
    
##迭代

    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }    