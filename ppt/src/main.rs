use std::thread;
use std::sync::Arc;

fn main() {
    
    /*let mut data = Box::new(1);
    for i in 0..3 {
        data = data.clone();
        thread::spawn( move ||{*data+=1;});
    }*/
    
    /*let x;                    // -+ x goes into scope
                              //  |
    {                         //  |
        let y = &5;           // ---+ y goes into scope
        let f = Foo { x: y }; // ---+ f goes into scope
        x = &f.x;             //  | | error here
    }                         // ---+ f and y go out of scope
                              //  |
    println!("{}", x);        //  |*/
    


    let x = Arc::new(5);
    let y = x.clone();
    
    

}

/*struct Foo<'a> {
    x: &'a i32,
}


struct Foo {
    f:i32,
}

fn main(){
    let y: &Foo;
    {
        let mut x = Foo{f:1};
        
        y = &x;
    }
    
    println!("{}", y.f);
}*/


/*struct Foo {
    f : Box<i32>,
}

struct Bar<'a> {
    foo : &'a Foo,
    doo : &'a Foo
}

fn main() {
    let mut a = Foo {f: Box::new(14)};

    let d : &Foo;

    { // block1
        let mut b = Foo {f: Box::new(13)};

        let bar = Bar{ foo : &a,doo : &b};
        println!("{}" ,  bar.foo.f);

        d = bar.foo;
    } // end of block1

    //a.f = Box::new(1);
    println!("{}" ,  d.f);
}*/

