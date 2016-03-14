fn main() {
    let mut x = 5;
    {
        let y = &mut x;
        //x= 6; //error
        *y += 1;
    }    
    
    println!("{}", x);
}

fn borrow_can_not_change()
{
    let x = 5;
    let y = &x;
    
    //x = 6;  //error
    //*y = 6;  //error
}
