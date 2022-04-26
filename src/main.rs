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

    // Even though variables are mutable by default, they can also be constant
    // Constant variables must be type annotated unlike regular variables
    // 
    const TWENTY_FOUR_HOURS_IN_MINUTES: u32 = 24 * 60;
    
}
