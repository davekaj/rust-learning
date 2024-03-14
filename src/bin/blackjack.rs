use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use colored::*;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Welcome to Blackjack!");
    let mut balance = 100;
    loop {
        println!("--------------------------------------------------------------");
        println!("{} {}","Your balance: $".green(), balance.to_string().green());
        if balance == 0 {
            println!("{}", "You're out of money. Game over!".red());
            break;
        }

        println!("Place your bet (or 'q' to quit):");
        let mut bet_input = String::new();
        io::stdin().read_line(&mut bet_input).expect("Failed to read line");
        if bet_input.trim() == "q" {
            break;
        }
        let mut bet: u32 = bet_input.trim().parse().expect("Invalid bet amount");
        if bet > balance {
            println!("Insufficient balance. Please enter a lower bet.");
            continue;
        }

        play_blackjack(&mut balance, &mut bet);
    }
}

fn play_blackjack(balance: &mut u32, bet: &mut u32) {
    let mut deck = create_deck();
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
    let mut player_hands = vec![vec![deck.pop().unwrap(), deck.pop().unwrap()]];
    let mut dealer_hand: Vec<String> = vec![deck.pop().unwrap(), deck.pop().unwrap()];
    println!("{}{}", "Dealer shows: ".yellow(), format_card(&dealer_hand[0]));

    let mut current_hand_index = 0;
    while current_hand_index < player_hands.len() {
        let mut player_turn = true;
        while player_turn {
            println!("Hand {}: {}", current_hand_index + 1, format_hand(&player_hands[current_hand_index]));
            println!("{}{}","Your Score: ".blue(), hand_value(&player_hands[current_hand_index]).to_string().blue());
            println!("Do you want to (h)it, (s)tand, (d)ouble down, or (p)lit?\n");
            let mut action = String::new();
            io::stdin().read_line(&mut action).expect("Failed to read line");
            match action.trim() {
                "h" => {
                    player_hands[current_hand_index].push(deck.pop().unwrap());
                    println!("You hit.");
                    if hand_value(&player_hands[current_hand_index]) > 21 {
                        println!("{}", "Bust! You lose this hand.".red());
                        current_hand_index += 1;
                        player_turn = false;
                    }
                }
                "s" => {
                    println!("You stand.");
                    player_turn = false;
                    current_hand_index += 1;
                }
                "d" => {
                    if *balance < *bet * 2 {
                        println!("Insufficient balance to double down.");
                    } else {
                        player_hands[current_hand_index].push(deck.pop().unwrap());
                        println!("You double down.");
                        if hand_value(&player_hands[current_hand_index]) > 21 {
                            println!("{}", "Bust! You lose this hand.".red());
                            println!("");
                            *balance -= *bet * 2;
                            current_hand_index += 1;
                            player_turn = false;
                        } else {
                            println!("Hand {}: {}", current_hand_index + 1, format_hand(&player_hands[current_hand_index]));
                            println!("{}{}","Your Score: ".blue(), hand_value(&player_hands[current_hand_index]).to_string().blue());
                            println!("");
                            player_turn = false;
                            current_hand_index += 1;
                            *bet *= 2;
                        }
                    }
                }
                "p" => {
                    if player_hands.len() == 1 && can_split(&player_hands[0]) {
                        let mut new_hand = vec![player_hands[0].pop().unwrap()];
                        new_hand.push(deck.pop().unwrap());
                        player_hands.push(new_hand);
                        player_hands[0].push(deck.pop().unwrap());
                        println!("You split your hand.");
                    } else {
                        println!("You cannot split your hand.");
                    }
                }
                _ => println!("Invalid input. Please enter 'h' to hit, 's' to stand, 'd' to double down, or 'p' to split."),
            }
        }
    }

    while hand_value(&dealer_hand) < 17 {
        println!("Dealer is thinking...");
        thread::sleep(Duration::from_secs(2)); // Delay for 2 seconds
        dealer_hand.push(deck.pop().unwrap());
        println!("Dealer hits.");
        println!("Dealer's hand: {}\n", format_hand(&dealer_hand));
    }

    let dealer_score = hand_value(&dealer_hand);
    println!("Dealer's score: {}", dealer_score.to_string().yellow());
    let mut player_wins = 0;
    let mut player_loses = 0;
    let mut player_ties = 0;
    for hand in &player_hands {
        let player_score = hand_value(hand);
        println!("Your score: {}", player_score.to_string().blue());
        if player_score == 21 && hand.len() == 2 && dealer_score != 21 {
            println!("{}", "Blackjack! You win this hand!".green());
            player_wins += 1;
            *balance += (*bet * 3) / 2;
        } else if dealer_score == 21 && dealer_hand.len() == 2 && player_score != 21 {
            println!("{}", "Dealer has Blackjack. You lose this hand.".red());
            player_loses += 1;
        } else if player_score > 21 {
            println!("{}", "Bust! You lose this hand.".red());
            player_loses += 1;
        } else if dealer_score > 21 || player_score > dealer_score {
            println!("{}", "You win this hand!".green());
            player_wins += 1;
        } else if player_score == dealer_score {
            println!("It's a tie!");
            player_ties += 1;
        } else {
            println!("{}", "Dealer wins this hand".red());
            player_loses += 1;
        }
    }

    if player_wins > 0 {
        println!("{}{}", player_wins.to_string().green(), " hand(s) won!".green());
        *balance += *bet * player_wins as u32;
    }
    if player_loses > 0 {
        println!("{}{}", player_loses.to_string().red(), " hand(s) lost.".red());
        *balance -= *bet * player_loses as u32;
    }
    if player_ties > 0 {
        println!("You tied {} hand(s).", player_ties);
    }
}
fn format_hand(hand: &[String]) -> String {
    hand.iter()
        .map(|card| format_card(card)) // Assuming format_card returns ColoredString
        .map(|colored_card| colored_card.to_string()) // Convert each ColoredString to String
        .collect::<Vec<String>>() // Now collecting into Vec<String>
        .join(", ")
}

fn format_card(card: &str) -> ColoredString {
    let suit = card.split_whitespace().last().unwrap();
    let mut colored_card: ColoredString = card.to_owned().bold(); // DK - Made bold just to convert it

    match suit {
        "Hearts" | "Diamonds" => colored_card = colored_card.red(),
        "Spades" | "Clubs" => colored_card = colored_card.black(),
        _ => {}
    }
    colored_card

}

fn can_split(hand: &[String]) -> bool {
    let rank = hand[0].split_whitespace().next().unwrap();
    hand.iter().all(|card| card.split_whitespace().next().unwrap() == rank)
}

fn create_deck() -> Vec<String> {
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];
    let mut deck = Vec::new();
    for &suit in &suits {
        for &rank in &ranks {
            deck.push(format!("{} of {}", rank, suit)); // Directly push owned String
        }
    }
    deck
}

fn hand_value(hand: &[String]) -> i32 {
    let mut value = 0;
    let mut has_ace = false;
    for card in hand {
        let rank = card.split_whitespace().next().unwrap();
        let card_value = match rank {
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "10" | "Jack" | "Queen" | "King" => 10,
            "Ace" => {
                has_ace = true;
                11
            }
            _ => 0,
        };
        value += card_value;
    }

    if has_ace && value > 21 {
        let mut new_value = value;
        for card in hand {
            let rank = card.split_whitespace().next().unwrap();
            if rank == "Ace" {
                new_value -= 10;
                // println!("Ace counted as 1"); - DK - not useful right now, it prints a lot, and it can be inferred by the user
                break;
            }
        }
        new_value
    } else {
        value
    }
}