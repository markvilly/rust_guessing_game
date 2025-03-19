// Requirements:
// Difficulty levels
// Random Number
// Time boundaries

use std::io;

fn main() {
    println!("Welcome to the guessing game!");

    let high_score: Option<u32> = None;
}

fn choose_difficulty() -> u32 {
    loop {
        println!("Choose a difficulty level: ");
        println!("1. Easy (1-50)");
        println!("2. Medium (1-100");
        println!("3. Hard (1-200");

        let mut difficulty: String = String::new();

        io::stdin()
            .read_line(&mut difficulty)
            .expect("Failed to read line");

        match difficulty.trim().parse() {
            Ok(1) => return 50,
            Ok(2) => return 100,
            Ok(3) => return 200,
            _ => println!("Invalid input, please enter a valid difficulty lever (1, 2, 3)"),
        };
    }
}
