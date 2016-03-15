#Mutability


##外部可变性

    use std::sync::Arc;

    let x = Arc::new(5);
    let y = x.clone();
    
这就是是“不可变性”的真正定义：
> 当有两个引用指向同一事物是安全的吗？在Arc<T>的情况下，是安全的：改变完全包含在结构自身内部。

##内部可变性

    use std::cell::RefCell;

    let x = RefCell::new(42);

    let y = x.borrow_mut();
    let z = x.borrow_mut();    
 RefCell使用borrow_mut()方法来分配它内部资源的&mut引   
    
##Mutability is a property of either a borrow (&mut) or a binding (let mut).

你不能拥有一个一些字段可变而一些字段不可变的结构体：

    struct Point {
        x: i32,
        mut y: i32, // nope
    }
    
    
结构体的可变性**位于它的绑定上**：

    struct Point {
        x: i32,
        y: i32,
    }

    let mut a = Point { x: 5, y: 6 };

    a.x = 10;

    let b = Point { x: 5, y: 6};

    b.x = 10; // error: cannot assign to immutable field `b.x`
    
    
然而，通过使用Cell<T>，你可以模拟字段级别的可变性：

    use std::cell::Cell;

    struct Point {
        x: i32,
        y: Cell<i32>,
    }

    let point = Point { x: 5, y: Cell::new(6) };

    point.y.set(7);

    println!("y: {:?}", point.y);