pub fn exec() {
    // Create a Vector with Initial Values:
    let mut colors = vec!["red", "green", "blue"];
    println!("Initial colors: {:?}", colors);

    // Remove an Element from a Vector:
    colors.pop(); // Removes the last element
    println!("Colors after pop: {:?}", colors);

    // Vector Slicing:
    let slice = &colors[0..2]; // Slice the vector
    println!("Sliced colors: {:?}", slice);

    // Create and Push to a Vector
    let mut scores: Vec<i32> = Vec::new();
    scores.push(85);
    scores.push(90);
    println!("Scores: {:?}", scores);

    // Accessing Vector Elements
    let first_score = scores[0];
    println!("First score: {}", first_score);

    //  Iterate Over a Vector
    for score in &scores {
        println!("Score: {}", score);
    }
}
