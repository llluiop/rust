#operators

Rust 允许有限形式的运算符重载。特定的运算符可以被重载。要支持一个类型间特定的运算符，你可以实现一个的特定的重载运算符的trait。

例如， + 运算符可以通过 Add 特性重载：

    use std::ops::Add;  //可以重载的特性在ops里说明

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point; //特性指定

        fn add(self, other: Point) -> Point { //特性实现
            Point { x: self.x + other.x, y: self.y + other.y }
        }
    }

    fn main() {
        let p1 = Point { x: 1, y: 0 };
        let p2 = Point { x: 2, y: 3 };

        let p3 = p1 + p2; //add

        println!("{:?}", p3);
    }


在 main 中，我们可以对我们的两个 Point 用 + 号，因为我们已经为 Point 实现了 Add<Output=Point> 。

有一系列可以这样被重载的运算符，并且所有与之相关的trait都在 std::ops 模块中。查看它的文档来获取完整的列表。

实现这些特性要遵循一个模式。让我们仔细看看 Add ：

    # mod foo {
    pub trait Add<RHS = Self> {   //rhs是个占位符，类似T，self是自身类型
        type Output;

        fn add(self, rhs: RHS) -> Self::Output;
    }
    # } 


这里总共涉及到3个类型：你 impl Add 的类型， RHS ，它默认是 Self ，和 Output 。对于一个表达式 let z = x + y ， x 是 Self 类型的， y 是 RHS ，而 z 是 Self::Output 类型。
# struct Point;
# use std::ops::Add;
impl Add<i32> for Point {
    type Output = f64;

    fn add(self, rhs: i32) -> f64 {
        // add an i32 to a Point and get an f64
# 1.0
    }
}


将允许你这样做：
let p: Point = // ...
let x: f64 = p + 2i32;
