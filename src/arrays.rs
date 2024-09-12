pub fn exec() {
    // Declare an Array
    let numbers = [1, 2, 3, 4, 5];
    println!("The third number is: {}", numbers[2]);

    //  Array with Explicit Type and Length
    let letters: [char; 3] = ['a', 'b', 'c'];
    println!("First letter: {}", letters[0]);

    // Array Initialization with Same Value:
    let zeros = [0; 5]; // Creates an array with five 0s
    println!("Array of zeros: {:?}", zeros);

    // Array with Different Data Types (Error Example):
    let mixed = [1, "two", 3.0]; // This would cause an error in Rust

    //Array Iteration
    for num in numbers.iter() {
        println!("Number: {}", num);
    }

    // Passing an Array to a Function:
    print_array(numbers);
}

fn print_array(arr: [i32; 3]) {
    for i in arr.iter() {
        println!("{}", i);
    }
}
