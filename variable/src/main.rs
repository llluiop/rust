fn main() {
    let x = 5;
    let x = "something";
    
    {
        let x = 5;
        
        let y = 10;
    }
    
    let y = "new";
}
