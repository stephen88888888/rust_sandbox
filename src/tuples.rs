//Tuple with Named Fields (Struct-like Tuple):
struct Point(i32, i32, i32);

// Unit-Like Struct:
struct AlwaysEqual;

pub fn exec() {
    // Declare a Tuple
    let person: (i32, f64, char) = (25, 6.1, 'M');
    println!("Age: {}, Height: {}, Gender: {}", person.0, person.1, person.2);

    // Destructure a Tuple
    let (age, height, gender) = person;
    println!("Age: {}, Height: {}, Gender: {}", age, height, gender);

    // Accessing Tuple Elements
    let coordinates: (i32, i32) = (10, 20);
    println!("X: {}, Y: {}", coordinates.0, coordinates.1);

    //Tuple with Named Fields (Struct-like Tuple):
    let origin = Point(0, 0, 0);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    // Swap Values Using Tuples:
    let (a, b) = (5, 10);
    println!("Before swap: a = {}, b = {}", a, b);
    let (a, b) = (b, a);
    println!("After swap: a = {}, b = {}", a, b);

    // Tuple as a Return Value:
    fn calculate(a: i32, b: i32) -> (i32, i32) {
        (a + b, a * b)
    }
    let (sum, product) = calculate(4, 5);
    println!("Sum: {}, Product: {}", sum, product);
}
