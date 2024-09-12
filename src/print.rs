pub fn show() {
    // Print a Greeting Message:
    println!("Hello, {}! Welcome to the Rust course.", "student");

    // Print a Math Operation Result:
    let result = 5 + 10;
    println!("The result of 5 + 10 is: {}", result);

    // Print with Alignment:
    println!("{:>8}", "right");

    // Print with Positional Arguments:
    println!("{0} is from {1} and {0} likes to {2}.", "Alice", "Wonderland", "explore");

    // Print with Named Arguments:
    println!(
        "{subject} {verb} {object}",
        subject = "The quick brown fox",
        verb = "jumps over",
        object = "the lazy dog"
    );

    // Print Debug Output:
    let tuple = (1, "hello", 4.5);
    println!("{:?}", tuple); // Debug format
}
