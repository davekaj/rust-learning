use rand::Rng; // Import Rng trait for random number generation
use std::io; // For accessing command line arguments
use std::process; // For terminating the program

fn main() {
    let total = roll_dice(); // Roll two dice and sum them

    loop {
        println!("Guess the total of two dice (2-12):"); // Prompt the user for input
        let mut guess = String::new(); // Create a new mutable string

        io::stdin()
            .read_line(&mut guess) // Read the user's input
            .expect("failed to read line"); // Handle any errors

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("ERROR: Please provide a number as a guess."); // Handle invalid input
                process::exit(1);
            }
        };

        if guess < 2 || guess > 12 {
            eprintln!("ERROR: Your guess must be between 2 and 12."); // Handle invalid input
            process::exit(1);
        }

        if guess == total {
            println!("YOU WIN!"); // User guessed correctly
            break; // Exit the loop
        } else {
            println!("Sorry, that's not correct. Try again!"); // Incorrect guess, loop will continue
        }
    }
}

fn roll_dice() -> i32 {
    let mut rng = rand::thread_rng(); // Get a random number generator
    let die1 = rng.gen_range(1..=6); // Roll the first die (1-6)
    let die2 = rng.gen_range(1..=6); // Roll the second die (1-6)
    die1 + die2 // Return the sum of the two rolls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Testing that the roll_dice function produces a valid total, over 1000 tries
    fn roll_dice_produces_valid_total() {
        let mut valid_range = true;
        for _ in 0..1000 {
            let total = roll_dice();
            if !(2..=12).contains(&total) {
                valid_range = false;
                break;
            }
        }
        assert!(valid_range, "roll_dice produced a total outside the valid range (2-12)");
    }
}
