fn main() {
    loop {
        println!("Loop!");
        break;
    }
    println!("------------");

    let mut number = 3;
    println!("While loop");
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("------------");

    println!("For loop");
    let a = [1, 2, 3, 4, 5, 6, 7];
    for element in a.iter() {
        println!("{}", element);
    }
}
