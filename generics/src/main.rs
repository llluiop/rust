fn takes_anything<T>(x: T) {
    // do something with x
  
    //println!("{}", x); //error! 即使注释掉主函数的调用依然报错，这点和c++不一样，似乎T类型需要有一个显式的能力规定
    
}

fn main() {
        
    let x:i32 = 5;
    takes_anything(x);
}

