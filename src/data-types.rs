pub fn exec() {
    // Declare an Integer
    let age: u8 = 30;
    println!("Age is: {}", age);

    // Declare a Float
    let temperature: f32 = 98.6;
    println!("Body temperature: {}°F", temperature);

    // Declare a Boolean
    let is_active: bool = true;
    println!("Is the system active? {}", is_active);

    // Declare a Character:
    let letter = 'R';
    println!("The letter is: {}", letter);

    // Declare an Unsigned Integer:
    let population: u64 = 7_800_000_000;
    println!("World population: {}", population);

    // Declare a Complex Data Type:
    let complex_number: (f64, f64) = (2.0, 3.0); // Tuple representing a complex number
    println!("Complex number: {} + {}i", complex_number.0, complex_number.1);
}
