fn main() {
    let mut a = 4;
    if a < 10 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    a = 13;
    if a < 10 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    a = 6;
    if a % 4 == 0 {
        println!("a divisible by 4");
    } else if a % 3 == 0 {
        println!("a divisible by 3");
    } else if a % 2 == 0 {
        println!("a divisible by 2");
    } else {
        println!("a is not divisible by 4, 3, 2");
    }
}
