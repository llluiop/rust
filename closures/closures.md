#closures

##基础语法

    fn  plus_one_v1   (x: i32) -> i32 { x + 1 }  //函数
    let plus_one_v2 = |x: i32| -> i32 { x + 1 }; //非推荐写法
    let plus_one_v3 = |x: i32|          x + 1  ; //一般写法
    
##闭包环境

    let mut num = 5;
    let plus_num = |x: i32| x + num;  //对外部变量num的引用，不可变引用
    
    //let y = &mut num; //这会引入一个错误，因为num已经在闭包里被不可变借用,解决方法是让
                          闭包出作用域
                          
                          
    let nums = vec![1, 2, 3];
    let takes_nums = || nums;         //取得所有权
    //let z = nums; //error
    
    
##move闭包

    let num = 5;

    let owns_num = move |x: i32| x + num;     //通过move，来获取num的一个拷贝
    
    
    注意下面：
    
    let mut num = 5;
    {
        let mut add_num = |x: i32| num += x;   //这里不使用move，必须使用作用域，使得下面的assert_eq就可以访问num
        add_num(5);
    }

    assert_eq!(10, num);                   //num值发生变化
    
    /****************/
    
    let mut num = 5;
    //{
        let mut add_num = move |x: i32| num += x;  //使用move，所以作用域不在也不影响下面的访问

        add_num(5);
    //}

    assert_eq!(5, num);  //值还是5
    
##闭包作为参数（Taking closures as arguments）    

    fn call_with_one<F>(some_closure: F) -> i32
        where F : Fn(i32) -> i32 {  //F的traits约定,注意Fn关键字

        some_closure(1)
    }

    let answer = call_with_one(|x| x + 2);

    动态分发：
    
    fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
        some_closure(1)
    }

    let answer = call_with_one(&|x| x + 2);
    assert_eq!(3, answer);
    
##函数指针和闭包

函数指针像一个没有环境的闭包，可以传递函数指针给期待闭包参数的函数：

    fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
        some_closure(1)
    }

    fn add_one(i: i32) -> i32 {
        i + 1
    }

    let answer = call_with_one(&add_one);
    
##返回闭包

    fn factory() -> Box<Fn(i32) -> i32> {
        let num = 5;

        Box::new(move |x| x + num)
    }
    # fn main() {
    let f = factory();

    let answer = f(1);
    assert_eq!(6, answer);
    # }    

