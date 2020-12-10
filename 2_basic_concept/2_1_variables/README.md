# Variables

- Declare: using keyword `let`
- In Rust, variables are <b>immutable</b> by default. 
```Rust
let x = 5;
println!("The value of x = {}", x);
x = 6; // This line is error. Can not change value of x
println!("The value of x = {}", x);
```

- Using keyword `mut` to declare a mutable variable
```Rust
let x = 5;
println!("The value of x = {}", x); // The value of x = 5
x = 6; 
println!("The value of x = {}", x); // The value of x = 6
```