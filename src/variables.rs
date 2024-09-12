pub fn exec() {
    // Declare a Variable with Inference:
    let temperature = 72.5; // Rust infers it as f64
    println!("Temperature: {}", temperature);

    // Immutable Variable:
    let name = "Rust";
    println!("Programming language: {}", name);
    name = "C++"; // This would cause a compile-time error

    // Declare a Mutable Variable:
    let mut score = 100;
    score += 50;
    println!("Updated score: {}", score);

    // Shadow a Variable:
    let x = 2;
    let x = x * 5;
    println!("Value of x: {}", x);

    // Declare a Constant:
    const PI: f64 = 3.14159;
    println!("The value of PI is: {}", PI);

    // Multiple Variable Declarations:
    let (x, y, z) = (10, 20, 30);
    println!("x: {}, y: {}, z: {}", x, y, z);
}
