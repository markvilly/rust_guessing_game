// Requirements:
// Difficulty levels
// Random Number
// Time boundaries

use std::io;
use rand::{rng, Rng};
use std::time::{Instant, Duration};
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guessing game!");

    let mut high_score: Option<(u32, Duration)> = None;

    let difficulty: u32 = choose_difficulty();
    let (secret_number, range) = generate_secret_number(difficulty);

    println!("I am thinking of a number between 1 and {}.", range);

    let mut guess_count: i32 = 0;
    let start_time: Instant = Instant::now();


    // the player input:

    loop {
        println!("Pick a guess...");

        let mut guess: String = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // parse the string back into a int.

        let guess_num: u32 = match guess.trim().parse() {
            Ok (num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        guess_count += 1;

        println!("You guessed: {}", guess_num);

        match guess_num.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                let duration: Duration = start_time.elapsed();
                println!("You win! you took {} guesses and {:?} seconds.", guess_count, duration.as_secs())
            }
        }

    }

    
}

fn choose_difficulty() -> u32 {
    loop {
        println!("Choose a difficulty level: ");
        println!("1. Easy (1-50)");
        println!("2. Medium (1-100)");
        println!("3. Hard (1-200)");

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

fn generate_secret_number(range: u32)-> (u32, u32){
    let secret_number = rand::rng().random_range(1..=range);

    (secret_number, range)
}