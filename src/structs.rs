// Defining a Struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

// Structs with methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn exec() {
    let user1 = User {
        username: String::from("samuel"),
        email: String::from("samuel@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("Username: {}", user1.username);

    // Update Struct Syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("User2 email: {}", user2.email);

    // Tuple Struct
    let black = Color(0, 0, 0);
    println!("Color: ({}, {}, {})", black.0, black.1, black.2);

    // Use method from struct
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
