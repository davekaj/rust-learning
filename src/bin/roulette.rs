use colored::*;
use rand::Rng; // Import Rng trait for random number generation
use std::io; // For accessing command line arguments
use std::process; // For terminating the program // For colored text

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

fn main() {
    print_roulette_table();
    println!("Welcome to the roulette table!");
    println!("What would you like to bet on? \n - Color (c)\n - Parity (Even/Odd) (p)\n - 1-18/19-36 (h)\n - Dozen (d)\n - Column (co)\n - Number (n)");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    choice = choice.trim().to_lowercase();

    match choice.as_str() {
        "c" => play_color(),
        "p" => play_parity(),
        "h" => play_half(),
        "d" => play_dozen(),
        "co" => play_column(),
        "n" => play_number(),
        _ => {
            println!("Invalid choice");
        }
    };
}

fn spin_table() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=37)

    // For testing purposes, we uncomment these
    // return 37;
    // return 0;
}

fn play_roulette<G, W>(prompt: &str, validate_guess: G, win_condition: W)
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
    } else {
        println!("{}", "Sorry, you lost.".red());
    }
}

fn play_color() {
    let validate_guess = || get_valid_input(1, 2, false);
    let win_condition = |guess, result| get_color_num(result) == guess;
    play_roulette("Bet on Red (1) or Black (2)", validate_guess, win_condition);
}

fn play_parity() {
    let validate_guess = || get_valid_input(1, 2, false);
    let win_condition = |guess, result| get_parity(result) == guess;
    play_roulette("Bet on Even (1) or Odd (2)", validate_guess, win_condition);
}

fn play_half() {
    let validate_guess = || get_valid_input(1, 2, false);
    let win_condition = |guess, result| (guess == 1 && result <= 18) || (guess == 2 && result > 18);
    play_roulette("Bet on 1-18 (1) or 19-36 (2)", validate_guess, win_condition);
}

fn play_dozen() {
    let validate_guess = || get_valid_input(1, 3, false);
    let win_condition = |guess, result| {
        (guess == 1 && result <= 12)
            || (guess == 2 && result > 12 && result <= 24)
            || (guess == 3 && result > 24)
    };
    play_roulette("Bet on 1-12 (1), 13-24 (2), or 25-36 (3)", validate_guess, win_condition);
}

fn play_column() {
    let validate_guess = || get_valid_input(1, 3, false);
    let win_condition = |guess, result| {
        (guess == 1 && result % 3 == 1)
            || (guess == 2 && result % 3 == 2)
            || (guess == 3 && result % 3 == 0)
    };
    play_roulette(
        "Bet on 1st (1), 2nd (2), or 3rd (3) column (see the ASCII art roulette table)",
        validate_guess,
        win_condition,
    );
}

fn play_number() {
    let validate_guess = || get_valid_input(0, 37, true);
    let win_condition = |guess: usize, result: usize| -> bool {
        guess == result
    };
    play_roulette("Bet on 0, 00, or 1 to 36", validate_guess, win_condition);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////// HELPER FUNCTIONS /////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

fn get_valid_input(min: usize, max: usize, bet_00: bool) -> usize {
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
            eprintln!("ERROR: Please provide a positive integer as a guess."); // Handle invalid input
            process::exit(1);
        }
    };

    if guess < min || guess > max {
        eprintln!("ERROR: Your guess must be between {} and {}", min, max);
        process::exit(1);
    }

    return guess;
}

fn get_parity(num: usize) -> usize {
    if num == 0 || num == 37 {
        return ZERO_OR_DOUBLE;
    } else if num % 2 == 0 {
        return EVEN;
    } else if num % 2 == 1 {
        return ODD;
    } else {
        panic!("Invalid number: {}", num);
    }
}

// 0 = green, 1 = red, 2 = black
fn get_color_num(num: usize) -> usize {
    if num == 0 || num == 37 {
        return ZERO_OR_DOUBLE;
    } else if RED_NUMBERS.contains(&num) {
        return RED;
    } else if num <= 37 {
        return BLACK;
    } else {
        panic!("Invalid number: {}", num);
    }
}

fn colorize_number(num: usize) -> ColoredString {
    match num {
        0 => ZERO.green(), // colorize_number "0" as green.
        37 => DOUBLE_ZERO.green(), // Special handling for "00" to display it correctly.
        _ if RED_NUMBERS.contains(&num) => format!("{}", num.to_string().red()).into(),
        _ => format!("{}", num.to_string().black()).into(),
    }
}

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
