# Functions
- Block of organized, reusable code
- Better modularity
- Code reuse
-----------------
Implicit return : don't add semi colon
```Rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```
Explicit return:
```rust
fn add(x: i32, y: i32) -> i32 {
    let mut result = x + y;
    result *= 100;
    return result;
}
```


