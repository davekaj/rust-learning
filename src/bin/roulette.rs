use rand::Rng; // Import Rng trait for random number generation
use std::io; // For accessing command line arguments
use std::process; // For terminating the program

fn main() {
    println!("Place your bet for roulette, 0, 00, and 1-36 are the numbers"); // Prompt the user for input
    let mut guess = String::new(); // Create a new mutable string
    io::stdin()
        .read_line(&mut guess) // Read the user's input
        .expect("failed to read line"); // Handle any errors
    if guess == "00".to_string() {
        guess = "37".to_string(); // Treat 00 as 37
    }

    let guess: usize = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("ERROR: Please provide a positive integer as a guess."); // Handle invalid input
            process::exit(1);
        }
    };
    println!("You guessed: {} {}", to_colour(get_colour(guess)), guess); // Print the user's guess
    // don't need to check if less than 0 because it's a usize
    if guess > 37 {
        eprintln!("ERROR: Your guess must be between 0 and 36, or 00"); // Handle invalid input
        process::exit(1);
    }

    let result = play_roulette();
    let colour = get_colour(result);

    if guess == result {
        println!("YOU WIN!"); // User guessed correctly
    } else {
        println!("Sorry, you lost, it landed on {} {}", to_colour(colour), result);
    }
}

fn play_roulette() -> usize {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..=37);
}

// 0 = black, 1 = red, 2 = green
fn get_colour(num: usize) -> usize {
    let red = [32, 19, 21, 25, 34, 27, 36, 30, 23, 5, 16, 1, 14, 9, 18, 7, 12, 3];

    if num == 0 || num == 37 {
        return 2;
    } else if red.contains(&num) {
        return 1;
    } else if num <= 37 {
        return 0;
    } else {
        panic!("Invalid number: {}", num);
    }
}

fn to_colour(num: usize) -> String {
    match num {
        0 => "Black".to_string(),
        1 => "Red".to_string(),
        2 => "Green".to_string(),
        _ => panic!("Invalid colour: {}", num),
    }
}