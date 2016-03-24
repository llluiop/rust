fn main() {
    let x = [1,2,3];
    let y = &x[..];
    
    let (a,b,c) = (1,2,3);
    
    let mut z = 5;
    let zz = &mut z;
    let p = *zz + 1;
    println!("{}", p);
    
    let test;
    test = 5;
}
