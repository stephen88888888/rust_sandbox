pub fn exec() {
    // Creating a Reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Pointers & References
    let s1 = String::from("Hello");
    print_length(&s1);

    // Mutable Reference
    let mut s = String::from("hello");
    append_text(&mut s);
    println!("{}", s);

    // Dangling Reference Error
    // This code will cause an error
    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn print_length(s: &String) {
    println!("The length of '{}' is {}.", s, s.len());
}

fn append_text(s: &mut String) {
    s.push_str(", world!");
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
