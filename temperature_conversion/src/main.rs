use std::io;

fn main() {
    let mut temperature = String::new();
    let temperature: f64 = loop {
        let mut unit = String::new();
        println!("Convert to Farenheit or Celsius? (assumes the opposite is input) (f/c)");
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line.");
        let unit: char = match unit.trim().parse::<char>() {
            Ok(unit) => {
                if unit == 'f' || unit == 'c' {
                    unit
                } else {
                    println!("Invalid char entered, Retry!");
                    continue;
                }
            }
            Err(_) => {
                println!("Invalid char entered, Retry!");
                continue;
            }
        };

        println!("Enter the temperature:");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f64 = match temperature.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if unit == 'f' {
            break temperature * 9.0 / 5.0 + 32.0;
        } else if unit == 'c' {
            break (temperature - 32.0) * 5.0 / 9.0;
        }
    };
    println!("The converted temperature is: {}", temperature);
}
