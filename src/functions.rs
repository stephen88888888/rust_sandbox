pub fn exec() {
    greet("Rustacean");

    let sum = add(5, 3);
    println!("Sum is: {}", sum);

    display_info("Alice", 30);

    greetOptional(Some("Alice"));
    greetOptional(None);

    let (square, cube) = square_and_cube(3);
    println!("Square: {}, Cube: {}", square, cube);
}

// Basic Function Definition
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function with Return Value
fn add(a: i32, b: i32) -> i32 {
    a + b
}

//  Function with Multiple Parameters
fn display_info(name: &str, age: u32) {
    println!("Name: {}, Age: {}", name, age);
}

// Function with an Optional Parameter (using Option):
fn greetOptional(name: Option<&str>) {
    match name {
        Some(n) => println!("Hello, {}!", n),
        None => println!("Hello, stranger!"),
    }
}

// Function Returning a Tuple:
fn square_and_cube(x: i32) -> (i32, i32) {
    (x * x, x * x * x)
}
