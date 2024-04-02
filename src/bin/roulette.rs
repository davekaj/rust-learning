use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;


// Constants for better readability
const ZERO: &str = "0";
const DOUBLE_ZERO: &str = "00";
const THIRTY_SEVEN: &str = "37";
const ZERO_OR_DOUBLE: usize = 0;
const RED: usize = 1;
const BLACK: usize = 2;
const EVEN: usize = 1;
const ODD: usize = 2;
const RED_NUMBERS: [usize; 18] = [
    1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36,
];

/// The entry point of the roulette game application.
/// Initializes player balance to $100 and runs the main game loop.
/// The game loop continues until the player's balance is greater than 0.
/// It handles betting input, choice of bet, and calculates win/loss.
fn main() {
    print_roulette_table();
    let mut balance: i32 = 100; // Player starts with $100
    println!(
        "{}{}.",
        "Welcome to the roulette table! To start you have $".green(),
        balance.to_string().green()
    );
    while balance > 0 {
        println!("How much would you like to bet?");
        let mut bet_str = String::new();
        io::stdin()
            .read_line(&mut bet_str)
            .expect("Failed to read line");
        let bet: i32 = bet_str.trim().parse().unwrap_or(0);

        if bet > balance || bet <= 0 {
            println!(
                "Invalid bet amount. You have ${}, and you tried to bet ${}.",
                balance, bet
            );
            continue;
        }

        println!("What would you like to bet on? \n - Color (c)\n - Parity (Even/Odd) (p)\n - 1-18/19-36 (h)\n - Dozen (d)\n - Column (co)\n - Number (n)");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        choice = choice.trim().to_lowercase();

        let win = match choice.as_str() {
            "c" => play_color(),
            "p" => play_parity(),
            "h" => play_half(),
            "d" => play_dozen(),
            "co" => play_column(),
            "n" => play_number(),
            _ => {
                println!("Invalid choice, please enter one of the correct options.");
                continue;
            }
        };

        if win > 0 {
            println!("You won ${} on a bet of ${}!\n\n", bet * win, bet);
            balance += bet * win;
        } else {
            println!("You lost ${}.\n\n", bet);
            balance -= bet;
        }
        println!(
            "{}{}.",
            "You now have $".green(),
            balance.to_string().green()
        );
        if balance <= 0 {
            println!("You've run out of money. Game over!");
        }
    }
}

/// Simulates spinning the roulette table and returns the result.
/// Delays the simulation for 2 seconds to mimic the spinning and then generates
/// a random number between 1 and 37 inclusive to represent the roulette result.
///
/// Returns:
/// A usize representing the slot number where the ball landed.
fn spin_table() -> usize {
    println!("Spinning..........");
    thread::sleep(Duration::from_secs(2)); // Delay for 2 seconds
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=37)

    // For testing purposes, we uncomment these
    // return 37;
    // return 0;
}

/// Executes a round of roulette based on the player's bet and checks if the player wins.
///
/// Parameters:
/// - `prompt`: The message displayed to the player indicating what to bet on.
/// - `validate_guess`: A closure that validates and returns the player's guess.
/// - `win_condition`: A closure that determines if the player's guess wins based on the result.
///
/// Returns:
/// - `true` if the player wins the round.
/// - `false` otherwise.
fn play_roulette<G, W>(prompt: &str, validate_guess: G, win_condition: W) -> bool
where
    G: Fn() -> usize,
    W: Fn(usize, usize) -> bool,
{
    println!("{}", prompt.yellow());
    let guess = validate_guess();
    let result = spin_table();
    println!("Result: {}", colorize_number(result));

    if win_condition(guess, result) {
        println!("{}", "YOU WIN!".green());
        return true;
    }
    println!("{}", "Sorry, you lost.".red());
    return false;
}

/// Facilitates betting on the color outcome of the spin.
///
/// Returns:
/// - `1` if the player wins (the payout is 1:1).
/// - `0` if the player loses.
fn play_color() -> i32 {
    let validate_guess = || get_valid_input(1, 2, false);
    let win_condition = |guess, result| get_color_num(result) == guess;
    let win = play_roulette("Bet on Red (1) or Black (2)", validate_guess, win_condition);
    if win { 1 } else { 0 }
}

/// Facilitates betting on the parity outcome of the spin.
///
/// Returns:
/// - `1` if the player wins (the payout is 1:1).
/// - `0` if the player loses.
fn play_parity() -> i32 {
    let validate_guess = || get_valid_input(1, 2, false);
    let win_condition = |guess, result| get_parity(result) == guess;
    let win = play_roulette("Bet on Even (1) or Odd (2)", validate_guess, win_condition);
    if win { 1 } else { 0 }
}

/// Facilitates betting on the 1-18 or 19-36 outcome of the spin.
/// 
/// Returns:
/// - `1` if the player wins (the payout is 1:1).
/// - `0` if the player loses.
fn play_half() -> i32 {
    let validate_guess = || get_valid_input(1, 2, false);
    let win_condition = |guess, result| (guess == 1 && result <= 18) || (guess == 2 && result > 18);
    let win = play_roulette(
        "Bet on 1-18 (1) or 19-36 (2)",
        validate_guess,
        win_condition,
    );
    if win { 1 } else { 0 }
}

/// Facilitates betting on the dozen outcome of the spin.
/// 
/// Returns:
/// - `2` if the player wins (the payout is 2:1).
/// - `0` if the player loses.
fn play_dozen() -> i32 {
    let validate_guess = || get_valid_input(1, 3, false);
    let win_condition = |guess, result| {
        (guess == 1 && result <= 12)
            || (guess == 2 && result > 12 && result <= 24)
            || (guess == 3 && result > 24)
    };
    let win = play_roulette(
        "Bet on 1-12 (1), 13-24 (2), or 25-36 (3)",
        validate_guess,
        win_condition,
    );
    if win { 2 } else { 0 }
}

/// Facilitates betting on the column outcome of the spin.
/// 
/// Returns:
/// - `2` if the player wins (the payout is 2:1).
/// - `0` if the player loses.
fn play_column() -> i32 {
    let validate_guess = || get_valid_input(1, 3, false);
    let win_condition = |guess, result| {
        (guess == 1 && result % 3 == 1)
            || (guess == 2 && result % 3 == 2)
            || (guess == 3 && result % 3 == 0)
    };
    let win = play_roulette(
        "Bet on 1st (1), 2nd (2), or 3rd (3) column (see the ASCII art roulette table)",
        validate_guess,
        win_condition,
    );
    if win { 2 } else { 0 }
}

/// Facilitates betting on a specific number outcome of the spin.
/// 
/// Returns:
/// - `35` if the player wins (the payout is 35:1).
/// - `0` if the player loses.
fn play_number() -> i32 {
    let validate_guess = || get_valid_input(0, 37, true);
    let win_condition = |guess: usize, result: usize| -> bool { guess == result };
    let win = play_roulette("Bet on 0, 00, or 1 to 36", validate_guess, win_condition);
    if win { 35 } else { 0 }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////// HELPER FUNCTIONS /////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Retrieves a valid input from the user within a specified range.
///
/// Parameters:
/// - `min`: The minimum acceptable value (inclusive).
/// - `max`: The maximum acceptable value (inclusive).
/// - `bet_00`: A flag indicating whether the special case of "00" is allowed.
///
/// Returns:
/// - A usize representing the user's validated input.
fn get_valid_input(min: usize, max: usize, bet_00: bool) -> usize {
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if bet_00 && guess.trim() == DOUBLE_ZERO.to_string() {
            guess = THIRTY_SEVEN.to_string(); // Treat 00 as 37
        }

        let guess: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("ERROR: Please provide a correct number for what bet you are making.");
                continue;
            }
        };

        if guess >= min && guess <= max {
            return guess;
        } else {
            eprintln!("ERROR: Your guess must be between {} and {}", min, max);
            continue;
        }
    }
}

/// Determines the parity (even/odd) of a given number.
///
/// Parameters:
/// - `num`: The number to check the parity of.
///
/// Returns:
/// - `ZERO_OR_DOUBLE` if the number is 0 or 00.
/// - `EVEN` if the number is even.
/// - `ODD` if the number is odd.
fn get_parity(num: usize) -> usize {
    if num == 0 || num == 37 {
        return ZERO_OR_DOUBLE;
    } else if num % 2 == 0 {
        return EVEN;
    } else if num % 2 == 1 {
        return ODD;
    } else {
        panic!("Invalid number: {}", num); // panic because this should never be reached
    }
}

/// Determines the color of a given slot number.
///
/// Parameters:
/// - `num`: The slot number to check the color of.
///
/// Returns:
/// - `ZERO_OR_DOUBLE` if the number is 0 or 00 (green).
/// - `RED` if the number is associated with a red slot.
/// - `BLACK` if the number is associated with a black slot.
fn get_color_num(num: usize) -> usize {
    if num == 0 || num == 37 {
        return ZERO_OR_DOUBLE;
    } else if RED_NUMBERS.contains(&num) {
        return RED;
    } else if num <= 37 {
        return BLACK;
    } else {
        panic!("Invalid number: {}", num); // panic because this should never be reached
    }
}

/// Applies coloring to the number based on its associated color in roulette.
///
/// Parameters:
/// - `num`: The roulette number to colorize.
///
/// Returns:
/// - A `ColoredString` with the number colored according to its roulette color
fn colorize_number(num: usize) -> ColoredString {
    match num {
        0 => ZERO.green(),
        37 => DOUBLE_ZERO.green(), // Special handling for "00" to display it correctly.
        _ if RED_NUMBERS.contains(&num) => format!("{}", num.to_string().red()).into(),
        _ => format!("{}", num.to_string().black()).into(),
    }
}

/// Prints the ASCII art representation of a roulette table to the console.
fn print_roulette_table() {
    println!("\n  Welcome to the roulette table!\n");
    println!(
        "\t+-------+------+\n\
         \t|   {}   |  {}  |\n\
         \t+----+----+----+\n\
         \t|  {} |  {} |  {} |\n\
         \t+----+----+----+\n\
         \t|  {} |  {} |  {} |\n\
         \t+----+----+----+\n\
         \t|  {} |  {} |  {} |\n\
         \t+----+----+----+\n\
         \t| {} | {} | {} |\n\
         \t+----+----+----+\n\
         \t| {} | {} | {} |\n\
         \t+----+----+----+\n\
         \t| {} | {} | {} |\n\
         \t+----+----+----+\n\
         \t| {} | {} | {} |\n\
         \t+----+----+----+\n\
         \t| {} | {} | {} |\n\
         \t+----+----+----+\n\
         \t| {} | {} | {} |\n\
         \t+----+----+----+\n\
         \t| {} | {} | {} |\n\
         \t+----+----+----+\n\
         \t| {} | {} | {} |\n\
         \t+----+----+----+\n\
         \t| {} | {} | {} |\n\
         \t+----+----+----+\n",
        ZERO.green(),
        DOUBLE_ZERO.green(),
        colorize_number(1),
        colorize_number(2),
        colorize_number(3),
        colorize_number(4),
        colorize_number(5),
        colorize_number(6),
        colorize_number(7),
        colorize_number(8),
        colorize_number(9),
        colorize_number(10),
        colorize_number(11),
        colorize_number(12),
        colorize_number(13),
        colorize_number(14),
        colorize_number(15),
        colorize_number(16),
        colorize_number(17),
        colorize_number(18),
        colorize_number(19),
        colorize_number(20),
        colorize_number(21),
        colorize_number(22),
        colorize_number(23),
        colorize_number(24),
        colorize_number(25),
        colorize_number(26),
        colorize_number(27),
        colorize_number(28),
        colorize_number(29),
        colorize_number(30),
        colorize_number(31),
        colorize_number(32),
        colorize_number(33),
        colorize_number(34),
        colorize_number(35),
        colorize_number(36)
    );
}
