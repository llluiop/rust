#method-syntax

##方法调用

    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    fn main() {
        let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
        println!("{}", c.area());
    }
    
    其实就是类似于class的概念，存在四种情况：
    fn foo(&self)
    fn foo(&mut self)
    fn foo(self)
    fn foo()  //关联函数
    
##关联函数

    struct Circle {
    x: f64,
    y: f64,
    radius: f64,
    }

    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Circle {  //类似于一个构造函数
            Circle {
                x: x,
                y: y,
                radius: radius,
            }
        }
    }

    fn main() {
        let c = Circle::new(0.0, 0.0, 2.0); //静态函数调用
    }    
    
    

##创建者模式

    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    struct CircleBuilder {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl CircleBuilder {
        fn new() -> CircleBuilder {
            CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
        }

        fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.x = coordinate;
            self
        }

        fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.y = coordinate;
            self
        }

        fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
            self.radius = radius;
            self
        }

        fn finalize(&self) -> Circle {
            Circle { x: self.x, y: self.y, radius: self.radius }
        }
    }

    fn main() {
        let c = CircleBuilder::new()
                    .x(1.0)
                    .y(2.0)
                    .radius(2.0)
                    .finalize();

        println!("area: {}", c.area());
        println!("x: {}", c.x);
        println!("y: {}", c.y);
    }    