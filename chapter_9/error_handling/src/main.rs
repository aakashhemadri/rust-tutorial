use std::fs::File;
use std::fs;
// use std::io::ErrorKind;
use std::io;
// use std::io::Read;
use std::error::Error;

#[allow(unused)]
fn main() ->  Result<(), Box<dyn Error>> {
    // Unrecoverable errors
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];

    // v[99];

    // Recoverable errors
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => {
    //         println!("The file read is {:#?}", file);
    //         file
    //     }
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    // let f = File::open("hello2.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello2.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");
    match read_username_from_file() {
        Ok(s) => println!("{}", s),
        Err(err) => panic!(),
    }
    let f = File::open("hello.txt")?;
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut f = File::open("hello2.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;

    // let mut s = String::new();
    // File::open("hello1.txt")?.read_to_string(&mut s)?;
    // Ok(s)
    fs::read_to_string("hello.txt")
}
