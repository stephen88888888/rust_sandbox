pub fn exec() {
    // Basic if-else
    let age = 18;
    if age >= 18 {
        println!("You are eligible to vote.");
    } else {
        println!("You are not eligible to vote.");
    }

    //  If-else with Multiple Conditions
    let number = 15;
    if number % 3 == 0 {
        println!("Number is divisible by 3.");
    } else if number % 5 == 0 {
        println!("Number is divisible by 5.");
    } else {
        println!("Number is not divisible by 3 or 5.");
    }

    //  Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("The value of number is: {}", number);

    // Ternary-like Condition (if-else in a let statement):
    let is_even = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {}", is_even);

    // Match Statement as a Conditional:
    let weather = "sunny";
    match weather {
        "rainy" => println!("Take an umbrella."),
        "sunny" => println!("Wear sunglasses."),
        _ => println!("Check the weather."),
    }

    // Nested if-else:
    let age = 20;
    if age < 18 {
        println!("Minor");
    } else if age >= 18 && age < 65 {
        println!("Adult");
    } else {
        println!("Senior");
    }
}
