#crates-mudules

#包装箱于模块

> 每个包装箱有一个隐含的根模块（root module）包含模块的代码

现在我们有了一个计划，让我们在代码中定义这些模块。让我们以用Cargo创建一个新包装箱作为开始：

    $ cargo new phrases
    $ cd phrases


如果你还记得，这会为我们生成一个简单的项目：

    $ tree .
    .
    ├── Cargo.toml
    └── src
        └── lib.rs

    1 directory, 2 files

 src/lib.rs 是我们包装箱的根，与上面图表中的 phrases 对应。

##定义模块

> 我们用 mod 关键字来定义我们的每一个模块。

##多文件包装箱

> mod可以按照一定的规律被拆分，存放于不同的目录，[参考](https://kaisery.gitbooks.io/rust-book-chinese/conten/content/Crates%20and%20Modules%20crate%20和模块.html)


##导入外部的包装箱

> 使用 extern crate mod-name; 语法，默认导入模块的函数都是私有的，要允许被使用，使用pub关键字
> 可以使用use关键字，将名字导入到我们本地作用域

    extern crate phrases;
    use phrases::english::greetings;
    
    
    如果你从同样的模块中导入多个名字，我们不必写多遍。Rust有一个简便的语法：
    
    use phrases::english::greetings;
    use phrases::english::farewells;

    我们可以使用这个简写：
    use phrases::english::{greetings, farewells};
    
    最好只导入模块，不要导入函数
    
##重导出

    extern crate phrases;

    use phrases::english::{greetings,farewells};
    use phrases::japanese;

    fn main() {
        println!("Hello in English: {}", greetings::hello());
        println!("Goodbye in English: {}", farewells::goodbye());

        println!("Hello in Japanese: {}", japanese::hello());  //之前应该是japanese::greetings::hello()***
        println!("Goodbye in Japanese: {}", japanese::goodbye());//之前应该是japanese::greetings::goodbye()***
    }    
    
    原因是，我们在mod japanese的定义中加入了：
    
    pub use self::greetings::hello;   //pub use关键字
    pub use self::farewells::goodbye;

    mod greetings;
    mod farewells;
    
##复杂的导出

Rust 提供了多种高级选项来让你的 extern crate 和 use 语句变得简洁方便。这是一个例子：


    extern crate phrases as sayings;

    use sayings::japanese::greetings as ja_greetings;
    use sayings::japanese::farewells::*;
    use sayings::english::{self, greetings as en_greetings, farewells as en_farewells};

    fn main() {
        println!("Hello in English; {}", en_greetings::hello());
        println!("And in Japanese: {}", ja_greetings::hello());
        println!("Goodbye in English: {}", english::farewells::goodbye());
        println!("Again: {}", en_farewells::goodbye());
        println!("And in Japanese: {}", goodbye());
    }


这里发生了什么？

首先， extern crate 和 use 都允许重命名导入的项。所以 crate 仍然叫“phrases”，不过这里我们以“sayings”来引用它。类似的，第一个 use 语句从 crate 中导入 japanese::greetings ，不过作为 ja_greetings 而不是简单的 greetings 。这可以帮助我们消除来自不同包中相似名字的项的歧义。

第二个 use 语句用了一个星号来引入 sayings::japanese::farewells 模块中的所有符号。如你所见之后我们可以不用模块标识来引用日语的 goodbye 函数。这类全局引用要保守使用。

第三个 use 语句需要更多的解释。它使用了“大括号扩展（brace expansion）”来将三条 use 语句压缩成了一条（这类语法对曾经写过 Linux shell 脚本的人应该很熟悉）。语句的非压缩形式应该是：
use sayings::english;
use sayings::english::greetings as en_greetings;
use sayings::english::farewells as en_farewells;


如你所见，大括号压缩了位于同一位置的多个项的 use 语句，而且在这里 self 指向这个位置。注意：大括号不用与星号嵌套或混合。
    


