use std::cmp::Ordering;
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn compare(a: i32, b: i32) -> () {
    return match a.cmp(&b) {
        Ordering::Less => println!("{} is less than {}", a, b),
        Ordering::Greater => println!("{} is greater than {}", a, b),
        Ordering::Equal => println!("{} is equal to {}", a, b),
    };
}
fn main() {
    let message = Message::Move { x: 10, y: 20 };
    match 5.cmp(&12) {
        Ordering::Less => println!("{} is less than {}", 5, 12),
        Ordering::Greater => println!("{} is greater than {}", 5, 12),
        Ordering::Equal => println!("{} is equal to {}", 5, 12),
    }
    let a = compare(4, 8);
    println!("{:?}", a);

    match message {
        Message::Quit => println!("Quit the game!"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("Message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: RGB({}, {}, {})", r, g, b),
    }
}
