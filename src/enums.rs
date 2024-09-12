// Define an Enum:
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Enum with Multiple Data Types:
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Enum with Data
enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Enum with Impl Block:
impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

// Using Enum in a Function:
enum Message {
    Start,
    Stop,
}

pub fn exec() {
    // Define an Enum
    let go = Direction::Up;

    // Enum with Data
    let msg = Message::Move { x: 10, y: 20 };

    // Enum Method Implementation
    let m = Message::Write(String::from("hello"));
    m.call();

    // Using Enum in a Function:
    process_message(Message::Start);
}

fn process_message(msg: Message) {
    match msg {
        Message::Start => println!("Starting..."),
        Message::Stop => println!("Stopping..."),
    }
}
