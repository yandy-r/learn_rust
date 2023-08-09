## Rust Notes

## Variables

- All variables are immutable by default. To make them mutable, use the `mut` keyword.
- To make a variable mutable, the variable must be declared with the `let` keyword.

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;

    assert_eq!(x, 6);
    println!("The value of x is: {}", x);

    let y = 10;
    println!("The value of y is: {}", y);
    y = 11; // error: cannot assign twice to immutable variable `y`
    assert_eq!(y, 11); // panic at 'assertion failed: `(left == right)` left: `10`, right: `11`'

    println!("The value of y is: {}", y);
}
```

### Using AS

- The `as` keyword is used to convert between types.

```rust
fn main() {
    // Using as to convert between types
    let a: u8 = 255;
    let b: u16 = a as u16;
    let c: u8 = b as u8;
    println!("a: {}, b: {}, c: {}", a, b, c); // a: 255, b: 255, c: 255
}
```
