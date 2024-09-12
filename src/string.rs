pub fn exec() {
    // Create a string
    let greeting = String::from("Hello, Rust!");
    println!("{}", greeting);

    //Concatenate Strings:
    let hello = String::from("Hello");
    let world = String::from("world!");
    let message = hello + ", " + &world;
    println!("{}", message);

    // Convert & Concatenate String Slices:
    let part1 = "Rust ";
    let part2 = "is awesome!";
    let message = [part1, part2].concat();
    println!("{}", message);

    // String Slicing
    let name = String::from("Samuel");
    let first_letter = &name[0..1];
    println!("The first letter of the name is: {}", first_letter);

    // Find the Length of a String:
    let phrase = String::from("Hello, Rust!");
    println!("The length of the phrase is: {}", phrase.len());

    // String Iteration:
    let word = String::from("rust");
    for c in word.chars() {
        println!("{}", c);
    }
}
