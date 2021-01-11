fn main() {
    println!("Compound Types: Tuples and Arrays");

    // Tuple Type
    // Optional type annotations
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring a tuple
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    // Accessing a tuple with the . operator

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    // Array types
    let _a = [1, 2, 3, 4, 5];

    // With type annotation
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // Setting initial value
    let _a = [3; 5]; // Same as writing `let a = [3, 3, 3, 3, 3]`

    // Accessing array elements
    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];

    // Runtime access error - Will compile but will panic! in runtime if you disable #[deny(unconditional_panic)]
    // let index = 10;

    // let element = a[index];

    // println!("The value of element is {}", element);
}
