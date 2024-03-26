use rand::Rng; // Import Rng trait for random number generation
use std::io; // For accessing command line arguments
use std::process; // For terminating the program
use colored::*; // For coloured text

fn main() {
    println!("What would you like to bet on? Colour (c), Parity (Even/Odd) (p), 1-18/19-36 (h), Dozen (d), Column (co), or Number (n)?");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    choice = choice.trim().to_lowercase();

    let guess = match choice.as_str() {
        "c" => verify_colour(),
        "p" => verify_parity(),
        "h" => verify_range(),
        "d" => verify_dozen(),
        "co" => verify_column(),
        "n" => verify_number(),
        _ => {
            println!("Invalid choice");
            0
        }
    };

    println!("You guessed: {}", colour_print(guess)); // Print the user's guess

    let result = play_roulette();
    let colour: usize = get_colour(result);
    println!("Result: {}", colour_print(result));

    if guess == result {
        println!("YOU WIN!"); // User guessed correctly
    } else {
        println!("Sorry, you lost");
    }
}

fn play_roulette() -> usize {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..=37);
}

fn verify_colour() -> usize {
    return 0;
}

fn verify_parity() -> usize {
    return 0;
}

fn verify_range() -> usize {
    return 0;
}

fn verify_dozen() -> usize {
    return 0;
}

fn verify_column() -> usize {
    return 0;
}

fn verify_number() -> usize {
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
    // don't need to check if less than 0 because it's a usize
    if guess > 37 {
        eprintln!("ERROR: Your guess must be between 0 and 36, or 00"); // Handle invalid input
        process::exit(1);
    }
    guess
}

// TODO
// - Even or Odd - use %
// - 1-18 or 19-36
// - Dozen - 1-12, 13-24, 25-36
// - Column - 1st, 2nd, 3rd
// - then implement betting with a balance
// - then implement multiple bets

// 0 = black, 1 = red, 2 = green
fn get_colour(num: usize) -> usize {
    let red = [
        32, 19, 21, 25, 34, 27, 36, 30, 23, 5, 16, 1, 14, 9, 18, 7, 12, 3,
    ];

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

fn colour_print(num: usize) -> ColoredString {
    let colour = get_colour(num); // Assuming `get_colour` function is defined elsewhere
    match colour {
        0 => ColoredString::from(format!("{} {}", "Black".black(), num.to_string().black())),
        1 => ColoredString::from(format!("{} {}", "Red".red(), num.to_string().red())),
        2 => ColoredString::from(format!("{} {}", "Green".green(), num.to_string().green())),
        _ => ColoredString::from(format!("Invalid colour: {}", num)),
    }
}