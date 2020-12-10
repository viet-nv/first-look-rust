fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(13);
    let v = add(13, 4);
    println!("v = {}", v);
    let t = explicit_return(13, 4);
    println!("t = {}", t);
}

fn another_function() {
    println!("Another Funtion");
}

fn another_function2(x: i32) {
    println!("The value of x = {}", x);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn explicit_return(x: i32, y: i32) -> i32 {
    let mut result = x + y;
    result *= 100;
    return result;
}