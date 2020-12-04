fn main() {
    let tup: (i32, f64, u8) = (500, 3.5, 1);
    let (x, y, z) = tup;    
    println!("x = {}", x);   
    println!("y = {}", y);   
    println!("z = {}", z);

    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("a = {}", a);   
    println!("b = {}", b);   
    println!("c = {}", c);
}