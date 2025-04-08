// use std::io::{self, Read};

// fn main() {
//     println!("Guess the number!");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_to_string(&mut guess)
//         .expect("Failed to read line");

//     // let int_guess = guess.trim().parse::<i32>().unwrap();

//     println!("You guessed: {}", guess);
// }

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    println!("Input: {}", input);
}
