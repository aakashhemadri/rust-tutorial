use std::io;

fn main() {
    println!("Enter a number n for which you'd like to find the nth Fibonacci Number:");
    let mut n = String::new();

    let num: u64 = loop {
        io::stdin().read_line(&mut n).expect("Failed to read line");

        match n.trim().parse::<u64>() {
            Ok(num) => break num,
            Err(_) => {
                n = "".to_string();
                continue;
            }
        };
    };

    println!("The Fibonnaci Number is: {}", fibonnaci(num));
}

fn fibonnaci(num: u64) -> u64 {
    if num <= 1 {
        num
    } else {
        fibonnaci(num - 1) + fibonnaci(num - 2)
    }
}
