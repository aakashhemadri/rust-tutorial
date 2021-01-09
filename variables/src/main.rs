fn main() {
    // Variables, Mutability & Shadowing
    println!("Variables, Mutability & Shadowing:");

    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // Shadowing
    // Great for transformations in a few places while maintaining immutability
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    // Useful to cast types, with same variable name,
    // by creating a new variable using let
    // Legal
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The number of spaces are {}", spaces);

    // Illegal - As type must be maintained.
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
