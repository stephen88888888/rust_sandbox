pub fn exec() {
    // Using a While Loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!");

    //  Using a For Loop
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Liftoff!");

    // Using a Loop
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    }
    println!("Counter stopped at {}", counter);

    // Using a Loop with Continue:
    for i in 1..10 {
        if i % 2 == 0 {
            continue;
        }
        println!("Odd number: {}", i);
    }

    // Infinite Loop with a Condition:
    let mut n = 0;
    loop {
        n += 1;
        if n > 5 {
            break;
        }
        println!("n = {}", n);
    }
}
