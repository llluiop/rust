#Enums

##枚举

> 一个enum是一个单独的类型。一个枚举的值可以匹配任何一个变量。
因为这个原因，枚举有时被叫做“集合类型”：枚举可能值的集合是每一个变量可能值的集合的总和:

        enum Message {
            Quit,
            ChangeColor(i32, i32, i32),
            Move { x: i32, y: i32 },
            Write(String),
        }       
        

        let x: Message = Message::Move { x: 3, y: 4 };
        
        一个枚举变量可以是它包含的任意一个类型，但同时也只能是一个类型。
        
        //不支持简单的解构操作，通过match来实现匹配
        fn process_color_change(msg: Message) {
            let Message::ChangeColor(r, g, b) = msg; // compile-time error
        }
        
        
##枚举构造

       enum Message {
           Write(String),
       }
       
       let x = Message::Write("Hello".to_string());
       
       or:
       
       fn(x:String) -> Message {
           Message::Write(x);
       }        
            