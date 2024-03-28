use colored::*;
use rand::Rng; // Import Rng trait for random number generation
use std::io; // For accessing command line arguments
use std::process; // For terminating the program // For colored text

fn main() {
    print_roulette_table();
    println!("What would you like to bet on? \n - Color (c)\n - Parity (Even/Odd) (p)\n - 1-18/19-36 (h)\n - Dozen (d)\n - Column (co)\n - Number (n)");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    choice = choice.trim().to_lowercase();

    match choice.as_str() {
        "c" => play_color(),
        "p" => play_parity(),
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
    return rng.gen_range(1..=37);
    // return 37;
}

fn play_color() {
    println!("Choose Red (r) or Black (b)");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess = guess.trim();
    if guess == "r" {
        println!("{}", "You bet on red!".red());
    } else if guess == "b" {
        println!("{}", "You bet on black!".black());
    } else {
        eprintln!("ERROR: Please provide 'r' or 'b' for color choice");
        process::exit(1);
    }

    let result = spin_table();
    let result_color = get_color(result);
    println!("Result: {}", color_print(result));
    if result_color == guess {
        println!("{}", "YOU WIN!".green());
    } else {
        println!("{}", "Sorry, you lost.".red());
    }
}

fn play_parity() {
    println!("Choose Even (e) or Odd (o)");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess = guess.trim();
    if guess == "e" {
        println!("You bet on Even!");
    } else if guess == "o" {
        println!("You bet on Odd!");
    } else {
        eprintln!("ERROR: Please provide 'e' or 'o' for parity choice");
        process::exit(1);
    }

    let result = spin_table();
    let result_parity = get_parity(result);
    println!("Result: {}", color_print(result));
    if result_parity == guess {
        println!("{}", "YOU WIN!".green());
    } else {
        println!("{}", "Sorry, you lost.".red());
    }
}

fn play_dozen() {
    println!("Choose 1-12 (1), 13-24 (2), or 25-36 (3)");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: usize = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("ERROR: Please provide a positive integer as a guess."); // Handle invalid input
            process::exit(1);
        }
    };

    if guess < 1 || guess > 3 {
        eprintln!("ERROR: Your guess must be between 1 and 3"); // Handle invalid input
        process::exit(1);
    }

    let result = spin_table();
    println!("Result: {}", color_print(result));
    if (guess == 1 && result <= 12)
        || (guess == 2 && result > 12 && result <= 24)
        || (guess == 3 && result > 24)
    {
        println!("YOU WIN!");
    } else {
        println!("Sorry, you lost");
    }
}

fn play_column() {
    println!("Choose 1st (1), 2nd (2), or 3rd (3) column (see the ASCII art roulette table)");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: usize = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("ERROR: Please provide a positive integer as a guess."); // Handle invalid input
            process::exit(1);
        }
    };

    if guess < 1 || guess > 3 {
        eprintln!("ERROR: Your guess must be between 1 and 3"); // Handle invalid input
        process::exit(1);
    }

    let result = spin_table();
    println!("Result: {}", color_print(result));
    if (guess == 1 && result % 3 == 1)
        || (guess == 2 && result % 3 == 2)
        || (guess == 3 && result % 3 == 0)
    {
        println!("YOU WIN!");
    } else {
        println!("Sorry, you lost");
    }
}

fn play_number() {
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

    println!("You bet on: {}", color_print(guess)); // Print the user's guess
    let result = spin_table();
    println!("Result: {}", color_print(result));
    if result == 1 {
        println!("YOU WIN!");
    } else {
        println!("Sorry, you lost");
    }
}

// 0 = black, 1 = red, 2 = green
fn get_color(num: usize) -> String {
    let red = [
        1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36,
    ];

    if num == 0 || num == 37 {
        return "g".to_string();
    } else if red.contains(&num) {
        return "r".to_string();
    } else if num <= 37 {
        return "b".to_string();
    } else {
        panic!("Invalid number: {}", num);
    }
}

fn get_parity(num: usize) -> String {
    if num == 0 || num == 37 {
        return "zeros".to_string();
    } else if num % 2 == 0 {
        return "e".to_string();
    } else if num % 2 == 1 {
        return "o".to_string();
    } else {
        panic!("Invalid number: {}", num);
    }
}

fn color_print(num: usize) -> ColoredString {
    let color = get_color(num); // Assuming `get_color` function is defined elsewhere
    match color.as_str() {
        "b" => ColoredString::from(format!("{} {}", "Black".black(), num.to_string().black())),
        "r" => ColoredString::from(format!("{} {}", "Red".red(), num.to_string().red())),
        "g" => {
            if num == 37 {
                return ColoredString::from(format!("{} {}", "Green".green(), "00".green()));
            } else {
                return ColoredString::from(format!("{} {}", "Green".green(), "0".green()));
            }
        }
        _ => ColoredString::from(format!("Invalid color: {}", num)),
    }
}

fn colorize(number: u32) -> ColoredString {
    match number {
        1 | 3 | 5 | 7 | 9 | 12 | 14 | 16 | 18 | 19 | 21 | 23 | 25 | 27 | 30 | 32 | 34 | 36 => {
            number.to_string().red()
        }
        2 | 4 | 6 | 8 | 10 | 11 | 13 | 15 | 17 | 20 | 22 | 24 | 26 | 28 | 29 | 31 | 33 | 35 => {
            number.to_string().black()
        }
        _ => number.to_string().green(), // This will catch 0 and 00, but they're already handled outside this function.
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
        "0".green(),
        "00".green(),
        colorize(1),
        colorize(2),
        colorize(3),
        colorize(4),
        colorize(5),
        colorize(6),
        colorize(7),
        colorize(8),
        colorize(9),
        colorize(10),
        colorize(11),
        colorize(12),
        colorize(13),
        colorize(14),
        colorize(15),
        colorize(16),
        colorize(17),
        colorize(18),
        colorize(19),
        colorize(20),
        colorize(21),
        colorize(22),
        colorize(23),
        colorize(24),
        colorize(25),
        colorize(26),
        colorize(27),
        colorize(28),
        colorize(29),
        colorize(30),
        colorize(31),
        colorize(32),
        colorize(33),
        colorize(34),
        colorize(35),
        colorize(36)
    );
}
