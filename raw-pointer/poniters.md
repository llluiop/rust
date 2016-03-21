#raw-pointers

##裸指针

 *const T 和 *mut T 在Rust中被称为“裸指针”。
 
 
##基础

创建一个裸指针是非常安全的：

    let x = 5;
    let raw = &x as *const i32;  //*const T

    let mut y = 10;
    let raw_mut = &mut y as *mut i32;  //*mut T


然而，解引用它则不行。这个并不能工作：

    let x = 5;
    let raw = &x as *const i32;

    println!("raw points at {}", *raw);


它给出这个错误：

    error: dereference of raw pointer requires unsafe function or block [E0133]
     println!("raw points at {}", *raw);
                                  ^~~~


当你解引用一个裸指针，你要为它并不指向正确的地方负责。为此，你需要 unsafe ：

    let x = 5;
    let raw = &x as *const i32;

    let points_at = unsafe { *raw };

    println!("raw points at {}", points_at);
