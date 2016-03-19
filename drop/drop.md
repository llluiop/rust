#drop

Rust 标准库提供的特殊 trait， Drop 。
Drop  trait提供了一个当一个值离开作用域后运行一些代码的方法。例如：

    struct Firework {
        strength: i32,
    }

    impl Drop for Firework {
        fn drop(&mut self) {
            println!("BOOM times {}!!!", self.strength);
        }
    }

    fn main() {
        let firecracker = Firework { strength: 1 };
        let tnt = Firework { strength: 100 };
    }
    
这会输出：
    BOOM times 100!!!
    BOOM times 1!!!
    
    tnt比firecracker先离开作用域
    