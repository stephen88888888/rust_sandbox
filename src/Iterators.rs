pub fn exec() {
    // Mapping Over a Vector:
    let numbers = vec![1, 2, 3];
    let doubled: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .collect();
    println!("{:?}", doubled);

    //  Filtering a Vector:
    let numbers = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<i32> = numbers
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect();
    println!("{:?}", even_numbers);

    //  Using filter_map to Filter and Transform:
    let strings = vec!["tofu", "93", "18", ""];
    let numbers: Vec<i32> = strings
        .into_iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("{:?}", numbers);

    //  Chaining Iterators:
    let numbers = vec![1, 2, 3];
    let result: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .filter(|x| x > &3)
        .collect();
    println!("{:?}", result);

    //  Using find with an Iterator:
    let numbers = vec![1, 2, 3, 4];
    let result = numbers.iter().find(|&&x| x == 2);
    println!("{:?}", result);

    //  Reducing a Vector to a Single Value:
    let numbers = vec![1, 2, 3, 4];
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("{}", sum);
}
