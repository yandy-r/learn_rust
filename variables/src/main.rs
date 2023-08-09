fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;

    assert_eq!(x, 6);
    println!("The value of x is: {}", x);

    let y = 10;
    println!("The value of y is: {}", y);
    // y = 11; // error: cannot assign twice to immutable variable `y`
    assert_eq!(y, 11); // panic at 'assertion failed: `(left == right)` left: `10`, right: `11`'

    println!("The value of y is: {}", y);
}
