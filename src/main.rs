use rand::Rng; // Import Rng trait for random number generation
use std::env; // For accessing command line arguments
use std::process; // For terminating the program

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command line arguments
    if args.len() < 2 {
        eprintln!("Usage: {} <guess>", args[0]); // Show usage if no arguments are given
        process::exit(1); // Exit the program with an error code
    }

    let guess: i32 = match args[1].parse() { // Parse the first argument as an i32
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please provide a number as a guess."); // Handle invalid input
            process::exit(1);
        }
    };

    let total = roll_dice(); // Roll two dice and sum them

    if guess == total {
        println!("YOU WIN!"); // User guessed correctly
    } else {
        println!("Sorry, the correct total was {}. Try again!", total); // Incorrect guess
    }
}

fn roll_dice() -> i32 {
    let mut rng = rand::thread_rng(); // Get a random number generator
    let die1 = rng.gen_range(1..=6); // Roll the first die (1-6)
    let die2 = rng.gen_range(1..=6); // Roll the second die (1-6)
    die1 + die2 // Return the sum of the two rolls
}
