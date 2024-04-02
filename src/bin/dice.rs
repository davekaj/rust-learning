use rand::Rng;
use std::io;

fn main() {
    let total = roll_dice();
    println!("Guess the total of two dice (2-12):");

    loop {
        match get_guess() {
            Ok(guess) if guess == total => {
                println!("YOU WIN!");
                break;
            }
            Ok(_) => println!("Sorry, that's not correct. Try again!"),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn roll_dice() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6) + rng.gen_range(1..=6)
}

fn get_guess() -> Result<i32, &'static str> {
    let mut guess_str = String::new();
    if io::stdin().read_line(&mut guess_str).is_err() {
        return Err("Failed to read line");
    }
    let guess: i32 = match guess_str.trim().parse() {
        Ok(num) if (2..=12).contains(&num) => num,
        _ => return Err("ERROR: Please provide a number as a guess between 2 and 12."),
    };
    Ok(guess)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roll_dice_produces_valid_total() {
        for _ in 0..1000 {
            let total = roll_dice();
            assert!((2..=12).contains(&total), "roll_dice produced an invalid total: {}", total);
        }
    }
}
