fn main() {
    println!("Hello, world!");
    let s = String::from("Hello World");

    let slice = first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {}", slice);

    // Other slices

    let a = [1, 2, 3, 4, 5];

    let _slice = &a[1..3];
}

fn first_word(s: &str) -> &str {
    // Accept and return string slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
