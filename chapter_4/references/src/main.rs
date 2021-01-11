fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // change(&s1); // immutable borrowing is illegal

    let mut s = String::from("hello");

    change(&mut s);

    // Restriction of mutable borrowing? -> Only one mutable reference
    // can be made
    // Illegal
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // we can use curly brackets to create a new scope,
    // allowing for multiple mutable references, just not simultaneous ones:
    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;

    let mut _s = String::from("hello");

    let _r1 = &s; // no problem
    let _r2 = &s; // no problem

    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");

    // Legal as mutable references happen after the lifetime of r1,r2
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    //Dangling references
    // let reference_to_nothing = dangle();
    let _string = no_dangle();
}

// fn dangle() -> &String {
//     // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// fn change(some_string: &String) { We cannot change some_string as it is being borrowed
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
