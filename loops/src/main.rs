fn main() {
    // Infinte loop
    // loop {
    //     println!("again!");
    // }

    // Returning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // Conditional loops with while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Looping through a collection with for using while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    // Looping using for through a collection

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Iterating through a range

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
