fn main() {
    for (count,element) in (15..20).enumerate() {
        println!("count = {} and element = {}", count, element);
    }
    
    let lines = "hello\nworld".lines();
    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }
}
