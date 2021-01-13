fn main() {
    // Structs
    let mut user1 = build_user(
        String::from("email@example.com"),
        String::from("Someone Cool"),
    );
    user1.email = String::from("anotheremail@example.com");
    user1.username = String::from("Someone who is not cool");
    user1.sign_in_count += 1;
    user1.active = false;
    let _user2 = User {
        username: String::from("user2@example.com"),
        ..user1
    };
    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
