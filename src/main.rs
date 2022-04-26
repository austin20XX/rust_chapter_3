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

    // Shadowing? Can't think of what I would need this for yet but...
    // Shadowing vs Mutable: With shadowing the type can be changed, with a regular mutable variable it cannot
    //Since we are using the 'let' keyword to redeclare the variable, it's not an error even though the variable is immutable
    let shadowed = 5;
    let shadowed = shadowed + 1;

    {
        let shadowed = shadowed * 2;
        println!("The value of the shadowed variable in the inner scope is: {}", shadowed);
    }

    println!("The value of the shadowed variable is: {}", shadowed);

    // Example of shadowing's use
    let spaces = "   ";
    let spaces = spaces.len();
}
