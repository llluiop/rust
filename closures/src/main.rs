fn main() {
    let plus = |x:i32| x+1;
    println!("{}", plus(2));
    
    let plus_two = |x| {
    let mut result: i32 = x;  //x此时自身不可变

    result += 1;
    result += 1;

    result
    };
    
    let mut num = 5;
    let plus_num = |x: i32| x + num;  //对外部变量num的引用，不可变引用
    //let y = &mut num; //error
    
    let nums = vec![1, 2, 3];

    let takes_nums = || nums;
    
    //let z = nums; //error
    
    let mut num = 5;

    //{
        let mut add_num = move |x: i32| num += x;

        add_num(5);
    //}

    assert_eq!(10, num);    
    
    
    fn call_with_one<F>(some_closure: F) -> i32
        where F : Fn(i32) -> i32 {  //F的traits约定

        some_closure(1)
    }

    let answer = call_with_one(|x| x + 2);
    println!("{}", answer);

}


