fn main() {
        
    let mut s = "Hello".to_string(); // mut s: String
    println!("{}", s);

    s.push_str(", world.");
    println!("{}", s);
        
        
    let hachiko = "hi,忠犬ハチ公";

    for b in hachiko.as_bytes() {
        print!("{}, ", b);
    }

    println!("");

    for c in hachiko.chars() {  //遍历字符，包括中英文字符自动识别
        print!("{}, ", c);
    }
    
    let dog = "忠犬ハチ公";
    let hachi = &dog[0..3];
    //let hachi = &dog[0..2]; //error，中文三个字节

    let hello = "Hello ".to_string();
    let world = "world!";

    let hello_world = hello + world;
}
