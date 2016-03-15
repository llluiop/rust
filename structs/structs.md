#strcuts

##创建与初始化

    struct Point {
        x: i32,
        y: i32,
    }

    fn main() {
        let mut point = Point { x: 0, y: 0 };  //key:value

        point.x = 5;

        println!("The point is at ({}, {})", point.x, point.y);
    }
    
##可变

    struct Point {
        x: i32,
        y: i32,
    }

    struct PointRef<'a> {
        x: &'a mut i32,
        y: &'a mut i32,
    }

    fn main() {
        let mut point = Point { x: 0, y: 0 };

        {
            let r = PointRef { x: &mut point.x, y: &mut point.y };

            *r.x = 5;
            *r.y = 6;
        }

        assert_eq!(5, point.x);
        assert_eq!(6, point.y);
    }
    
##结构体更新

    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let mut point = Point3d { x: 0, y: 0, z: 0 };
    point = Point3d { y: 1, .. point };  //更新y字段，保留原来的point.x和point.z
    
    
##结构体元组

    struct Color(i32, i32, i32);  //匿名字段
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
当元组结构体只有一个元素时。我们管它叫新类型（newtype），因为你创建了一个与元素相似的类型：

    struct Inches(i32);
    let length = Inches(10);

    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);
    
如你所见，你可以通过一个解构let来提取内部的整型，就像我们在讲元组时说的那样，
let Inches(integer_length)给integer_length赋值为10。

##类单元结构体（Unit-like structs）

你可以定义一个没有任何成员的结构体：

    struct Electron;
    let x = Electron;
    
这样的结构体叫做“类单元”因为它与一个空元组类似，()，这有时叫做“单元”。它定义了一个新类型。
例如，一个库可能请求你创建一个实现了一个特定特性的结构来处理事件。
如果你并不需要在结构中存储任何数据，你可以仅仅创建一个类单元结构体。
