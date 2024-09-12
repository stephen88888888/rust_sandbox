use std::collections::HashMap;

pub fn exec() {
    // Creating and Inserting into a HashMap:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //  Accessing Values in a HashMap:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Score: {:?}", score);

    //Iterating Over a HashMap:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //  Updating a Value in a HashMap:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    //  Using the Entry API:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    //  Merging Two HashMaps:
    let mut map1 = HashMap::new();
    map1.insert("a", 1);

    let mut map2 = HashMap::new();
    map2.insert("b", 2);

    map1.extend(map2);

    println!("{:?}", map1);
}
