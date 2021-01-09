fn five() -> i32 {
    5
}

fn main() {
    println!("Hello, functions!");

    another_function(5, 6);

    // Statements & Expressions
    let _y = 6; // Statement

    // Illegal as statements do not return a value
    // let x = (let y = 6);

    // {}/Scope is an expression

    let _x = 5;

    let y = {
        let x = 3;
        x + 1 // the final line does not have a semicolon
              // indicating it's an expression returned to y
    };

    println!("The value of y is: {}", y);

    // Function with return values

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(five());

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
