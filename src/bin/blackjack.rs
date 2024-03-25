use colored::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Welcome to Blackjack!");
    let mut balance = 100;
    loop {
        println!("--------------------------------------------------------------");
        println!(
            "{} {}",
            "Your balance: $".green(),
            balance.to_string().green()
        );
        if balance == 0 {
            println!("{}", "You're out of money. Game over!".red());
            break;
        }

        println!("Place your bet (or 'q' to quit)");
        let mut bet_input = String::new();
        io::stdin()
            .read_line(&mut bet_input)
            .expect("Failed to read line");
        if bet_input.trim() == "q" {
            break;
        }
        let mut parts = bet_input.trim().split_whitespace();
        let mut bet: u32;
        let mut secret_mode = false;
        if let Some(first_part) = parts.next() {
            if first_part == "s" {
                secret_mode = true;
                if let Some(bet_str) = parts.next() {
                    bet = bet_str.parse().expect("Invalid bet amount");
                } else {
                    println!("Please provide a valid bet amount after 's'");
                    continue;
                }
            } else {
                bet = first_part.parse().expect("Invalid bet amount");
            }
        } else {
            println!("Please provide a valid bet amount");
            continue;
        }

        if bet > balance {
            println!("Insufficient balance. Please enter a lower bet.");
            continue;
        }

        play_blackjack(&mut balance, &mut bet, secret_mode);
    }
}

fn play_blackjack(balance: &mut u32, bet: &mut u32, secret: bool) {
    let mut deck = create_deck();
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
    let mut player_hands: Vec<Vec<String>>;
    let mut dealer_hand: Vec<String>;
    if secret {
        player_hands = vec![pick_cards(&mut deck, false)];
        print!("Pick dealers hand");
        dealer_hand = pick_cards(&mut deck, true);
    } else {
        player_hands = vec![vec![deck.pop().unwrap(), deck.pop().unwrap()]];
        dealer_hand = vec![deck.pop().unwrap(), deck.pop().unwrap()];
    }

    println!(
        "{}{}",
        "\nDealer shows: ".yellow(),
        format_card(&dealer_hand[0])
    );
    println!(
        "{}{}",
        "Dealer score: ".yellow(),
        hand_value(&dealer_hand[..1]).to_string().yellow() // Pass only the first card
    );

    let mut current_hand_index = 0;
    let mut all_hands_busted = false;
    while current_hand_index < player_hands.len() {
        let mut player_turn = true;
        while player_turn {
            let value = hand_value(&player_hands[current_hand_index]);
            if value == 21 {
                match player_hands[current_hand_index].len() {
                    2 => println!("{}", "Blackjack!\n".green()),
                    _ => println!("{}", "You have 21!\n".green()),
                }
                player_turn = false;
                current_hand_index += 1;
                continue;
            }
            print_player_hand(&player_hands[current_hand_index], current_hand_index);
            println!("Do you want to (h)it, (s)tand, (d)ouble down, or (p)lit?\n");
            let mut action = String::new();
            io::stdin()
                .read_line(&mut action)
                .expect("Failed to read line");
            match action.trim() {
                "h" => {
                    player_hands[current_hand_index].push(deck.pop().unwrap());
                    println!("You hit.");
                    if hand_value(&player_hands[current_hand_index]) > 21 {
                        print_player_hand(&player_hands[current_hand_index], current_hand_index);
                        println!("{}", "Bust! You lose this hand.".red());
                        current_hand_index += 1;
                        player_turn = false;
                        if player_hands.iter().all(|hand| hand_value(hand) > 21) {
                            all_hands_busted = true;
                        }
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
                            if player_hands.iter().all(|hand| hand_value(hand) > 21) {
                                all_hands_busted = true;
                            }
                        } else {
                            print_player_hand(&player_hands[current_hand_index], current_hand_index);
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

    if all_hands_busted {
        println!("All hands busted. You lose!");
        *balance -= *bet * player_hands.len() as u32;
        return;
    }

    let dealer_score = dealer_turn(&mut deck, &mut dealer_hand);

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
        println!(
            "{}{}",
            player_wins.to_string().green(),
            " hand(s) won!".green()
        );
        *balance += *bet * player_wins as u32;
    }
    if player_loses > 0 {
        println!(
            "{}{}",
            player_loses.to_string().red(),
            " hand(s) lost.".red()
        );
        *balance -= *bet * player_loses as u32;
    }
    if player_ties > 0 {
        println!("You tied {} hand(s).", player_ties);
    }
}

fn dealer_turn(deck: &mut Vec<String>, dealer_hand: &mut Vec<String>) -> i32 {
    while hand_value(&dealer_hand) < 17 {
        println!(
            "{}{}",
            "\nDealer shows: ".yellow(),
            format_hand(&dealer_hand)
        );
        println!(
            "{}{}",
            "Dealer score: ".yellow(),
            hand_value(&dealer_hand).to_string().yellow() // Pass only the first card
        );
        println!("Dealer is thinking...");
        thread::sleep(Duration::from_secs(3)); // Delay for 2 seconds
        dealer_hand.push(deck.pop().unwrap());
        println!("Dealer hits.");
        println!("Dealer's hand: {}", format_hand(&dealer_hand));
    }

    let dealer_score = hand_value(&dealer_hand);
    println!(
        "{}{}",
        "\nDealer shows: ".yellow(),
        format_hand(&dealer_hand)
    );
    println!("Dealer's score: {}", dealer_score.to_string().yellow());
    dealer_score
}

////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////// HELPER FUNCTIONS /////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO - Fails when wrong cards are chosen (it just continues, when it should ask again)
fn pick_cards(deck: &mut Vec<String>, dealer: bool) -> Vec<String> {
    let mut hand = Vec::new();
    for (i, card) in deck.iter().enumerate() {
        println!("{}: {}", i, card);
    }
    if dealer {
        println!("\nPick dealers hand by choosing the index of the 1st card, pressing enter, and then choosing the index of the 2nd card.");
    } else {
        println!("\nPick your hand by choosing the index of the 1st card, pressing enter, and then choosing the index of the 2nd card.");
    }
    let mut picked_indices = Vec::new();
    for _ in 0..2 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let index = input.trim().parse::<usize>().unwrap();
        if index >= deck.len() {
            println!("Invalid index. Please try again.");
            continue;
        }
        if picked_indices.contains(&index) {
            println!("You've already picked that card. Please try again.");
            continue;
        }

        picked_indices.push(index);
    }
    // Sort in descending order so that removing elements doesn't mess up the indices
    picked_indices.sort_by(|a, b| b.cmp(a));
    // clone cards into hand and remove them from deck
    for index in picked_indices {
        hand.push(deck[index].clone());
        deck.remove(index);
    }

    hand
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
    hand.iter()
        .all(|card| card.split_whitespace().next().unwrap() == rank)
}

fn create_deck() -> Vec<String> {
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
    ];
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

fn print_player_hand(hand: &[String], hand_index: usize) {
    println!(
        "{}{}{}{}",
        "Hand ".blue(),
        (hand_index + 1).to_string().blue(),
        ": ",
        format_hand(&hand)
    );
    println!(
        "{}{}",
        "Your Score: ".blue(),
        hand_value(&hand).to_string().blue()
    );
}
