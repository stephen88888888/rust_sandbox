// Basic Trait Implementation

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

// Generic Structs
struct Point<T> {
    x: T,
    y: T,
}

pub fn exec() {
    // Basic Trait Implementation
    let article = Article {
        title: String::from("Rust Traits"),
        content: String::from("A powerful feature in Rust."),
    };
    println!("{}", article.summarize());

    //  Using Generics in Functions
    let numbers = vec![10, 20, 5, 8];
    println!("The largest number is {}", largest(&numbers));

    //  Generic Structs
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("Integer point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float point: ({}, {})", float_point.x, float_point.y);

    //  Trait Bounds in Functions
}

//  Using Generics in Functions
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn print_summary<T: Summary>(item: T) {
    println!("{}", item.summarize());
}
