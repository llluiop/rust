#string

##字符串

> rust string以uft-8字节编码Unicode，不以null结尾并且可以包含null，主要有两种类型
的string：

* &str：字符串片段，类型为&‘ static str
        let s = "hello world";
        // "hello world"是一个字符串常量，字符串常量是一个静态分配的字符串切片，存储在编好的程序中，在程序运行的过程中一直存在。
        
        字符串常量可以跨行：        
        
        let s = "foo
        bar";
        assert_eq!("foo\n        bar", s);           
        
        let s = "foo\
        bar"; 
        assert_eq!("foobar", s);

* string：堆上分配的字符串，可以增长，通常由一个&str调用to_string转换而来：

        let mut s = "Hello".to_string(); // mut s: String
        println!("{}", s);

        s.push_str(", world.");
        println!("{}", s);


String 可以通过一个 & 强制转换为 &str ：

    fn takes_slice(slice: &str) {
        println!("Got: {}", slice);
    }

    fn main() {
        let s = "Hello".to_string();
        takes_slice(&s);
    }


##索引

因为字符串是有效*UTF-8*(中文使用三个字节编码，英文使用一个，所以索引无意义)编码的，它不支持索引：
let s = "hello";

println!("The first letter of s is {}", s[0]); // ERROR!!!


    let hachiko = "忠犬ハチ公";

    for b in hachiko.as_bytes() {
        print!("{}, ", b);
    }

    println!("");

    for c in hachiko.chars() {
        print!("{}, ", c);
    }

    println!("");
    
    
##切片

切片（Slicing）

你可以使用切片语法来获取一个字符串的切片：

    let dog = "hachiko";
    let hachi = &dog[0..5];


注意这里是字节偏移，而不是字符偏移。所以如下代码在运行时会失败：

    let dog = "忠犬ハチ公";
    let hachi = &dog[0..2];

    给出如下错误：
    thread '' panicked at 'index 0 and/or 2 in `忠犬ハチ公` do not lie on
    character boundary'
    
    
##连接（Concatenation）

如果你有一个 String ，你可以在它后面接上一个 &str ：

    let hello = "Hello ".to_string();
    let world = "world!";

    let hello_world = hello + world;


不过如果你有两个 String ，你需要一个 & ：

    let hello = "Hello ".to_string();
    let world = "world!".to_string();

    let hello_world = hello + &world;


这是因为 &String 可以自动转换为一个 &str 。这个功能叫做 Deref 转换。

    
