fn main() {
    // let number = 3;
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;
    // if number { // Illegal as type must be Boolean
    if number != 0 {
        println!("number was three");
    }

    // Multiple conditions with else if - Preferably use `match`

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // Illegal because conditional must result in same type across arms
    // let number = if condition { 5 } else { "six" };
    println!("The value of the number is: {}", number);
}
