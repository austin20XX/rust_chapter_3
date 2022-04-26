fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // This line would produce an error, because by default variables are immutable in rust
    // x = 6;
    println!("The new value of x is: {}", x);

    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 20; 
    println!("The value of y is: {}", y);
}
